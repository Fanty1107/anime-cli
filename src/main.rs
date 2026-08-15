mod db;
use std::env;
use rusqlite::Result as ResultSql;

use db::{add_data, init_db, new_db, show_db, Anime};

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
