use core::fmt;
use std::env;
use rusqlite::{Connection, Result as ResultSql};
use comfy_table::Table;

struct Anime{
    id: Option<u32>,
    nome: String,
    num_ep: u32,
}
impl Anime {
    fn new(n: &str, n_ep: u32)-> Self {
       Self
       {
           id: None,
           nome: n.to_string(),
           num_ep: n_ep} 
    }
}
impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Anime: {}, Eps: {}", self.nome, self.num_ep)
    }
}

fn new_db() -> ResultSql<Connection>{
   let conn = Connection::open("database.db")?;
   Ok(conn)
}
fn init_db(conn: &Connection) -> ResultSql<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS anime(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            episodios INTEGER NOT NULL
    )", 
    [],
    )?;
    Ok(())
}
fn add_data(conn: &Connection, anime: &mut Anime) -> ResultSql<bool>{
    let lines = conn.execute("INSERT OR IGNORE INTO anime (name, episodios) VALUES (?1,?2)",
    rusqlite::params![anime.nome, anime.num_ep],
    )?;
    if lines > 0{
    let genareted_id = conn.last_insert_rowid() as u32;
    anime.id = Some(genareted_id);
    Ok(true)
    }else {
    Ok(false)
    }
}
fn show_db(conn: &Connection) -> ResultSql<()>{
    let mut stmt = conn.prepare("SELECT id, name, episodios FROM anime")?;

    let anime_iter = stmt.query_map([], |row|{
        Ok((
            row.get::<_,i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, u32>(2)?,
        ))
    })?;
    let mut table = Table::new();
    table.set_header(vec!["ID", "Name", "Episodes"]);
    for anime in anime_iter{
        let(id,nome,eps) = anime?;
        table.add_row(vec![id.to_string(), nome, eps.to_string()]);
    }
    println!("{table}");
    Ok(())
}

fn main()-> ResultSql<()>{
    let argv: Vec<String> = env::args().collect();
    let connec = new_db()?;
    init_db(&connec)?; 

    if let Some(arg1) = argv.get(1){
       let mut ani = Anime::new(arg1, 12);
       let insert_ani = add_data(&connec, &mut ani)?;
       if insert_ani{
       show_db(&connec)?; 
       }
       else {
           println!("Anime already exists in your database: {}", ani.nome);
       }
    }
    else{
        println!("no args passed");
    }
    Ok(())
}
