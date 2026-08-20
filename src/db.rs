use crate::Anime;
use comfy_table::Table;
use rusqlite::{Connection, Result as ResultSql};
use std::env;
use std::fs;
use std::path::PathBuf;
pub fn new_db() -> ResultSql<Connection> {
    let home_dir = env::var("HOME").expect("Error: directory home not found");
    let mut db_dir = PathBuf::from(home_dir);
    db_dir.push(".anime_cli");
    fs::create_dir_all(&db_dir).expect("Error: could not create directory");
    db_dir.push("database.db");
    let conn = Connection::open(db_dir)?;
    Ok(conn)
}

pub fn init_db(conn: &Connection) -> ResultSql<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS anime(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            current_ep INTEGER NOT NULL,
            episodios INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn add_data(conn: &Connection, anime: &mut Anime) -> ResultSql<bool> {
    let lines = conn.execute(
        "INSERT OR IGNORE INTO anime (name, episodios, current_ep) VALUES (?1,?2,?3)",
        rusqlite::params![anime.nome, anime.num_ep, anime.cur_ep],
    )?;

    if lines > 0 {
        let generated_id = conn.last_insert_rowid() as u32;
        anime.id = Some(generated_id);
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn show_db(conn: &Connection) -> ResultSql<()> {
    let mut stmt = conn.prepare("SELECT id, name, current_ep, episodios FROM anime")?;

    let anime_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, u32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, u32>(2)?,
            row.get::<_, u32>(3)?,
        ))
    })?;

    let mut table = Table::new();
    table.set_header(vec!["ID", "Name", "Current_Ep", "Episodes"]);

    for anime in anime_iter {
        let (id, nome, cur_eps, eps) = anime?;
        table.add_row(vec![
            id.to_string(),
            nome,
            cur_eps.to_string(),
            eps.to_string(),
        ]);
    }

    println!("{table}");
    Ok(())
}
pub fn search_ani(conn: &Connection, termo: &str) -> ResultSql<Anime> {
    if let Ok(id_num) = termo.parse::<u32>() {
        conn.query_row(
            "SELECT id, name, current_ep, episodios FROM anime WHERE id = ?1",
            [id_num],
            |row| {
                Ok(Anime {
                    id: Some(row.get(0)?),
                    nome: row.get(1)?,
                    cur_ep: row.get(2)?,
                    num_ep: row.get(3)?,
                })
            },
        )
    } else {
        conn.query_row(
            "SELECT id, name, current_ep, episodios FROM anime WHERE name = ?1",
            [termo],
            |row| {
                Ok(Anime {
                    id: Some(row.get(0)?),
                    nome: row.get(1)?,
                    cur_ep: row.get(2)?,
                    num_ep: row.get(3)?,
                })
            },
        )
    }
}

pub fn update_ep(conn: &Connection, ani: &mut Anime, seen_eps: u32) -> ResultSql<()> {
    ani.cur_ep += seen_eps;
    conn.execute(
        "UPDATE anime SET current_ep = ?1 WHERE id = ?2 OR name = ?3",
        rusqlite::params![ani.cur_ep, ani.id, ani.nome],
    )?;
    Ok(())
}

pub fn remove_ani(conn: &Connection, ani: &Anime) -> ResultSql<bool> {
    let lines = conn.execute(
        "DELETE FROM anime WHERE id = ?1 OR name = ?2",
        rusqlite::params![ani.id, ani.nome],
    )?;
    if lines > 0 { Ok(true) } else { Ok(false) }
}
