use crate::Anime;
use crate::db;
use rusqlite::{Connection, Result as ResultSql};
use std::{env, io, path::Path, process};

pub const COMMUN_PATH: &str = "~/.anime_cli/database.db";
pub const HELP_TEXT: &str = "\
Usage: anime-cli <COMMAND> <ANIME_NAME> [ARGS]

Commands:
  add <name> <total_eps>   Adds a new anime to the database.
  up <name>                Updates the watched episodes for a specific anime.
  rm <name>                Removes an anime from the database.
  show/-s                  Show the local database.
  -Sd                      Show database file location

Examples:
  anime-cli add \"One Piece\" 1070
  *****     up \"Naruto\"
  *****     up \"id_anime\"
  *****     rm \"Bleach\"
  *****     rm \"id_anime\"
  *****     show/-s
  *****     -Sd
  ";
pub fn print_err() {
    eprintln!("Error: command unknow");
    eprintln!("Use 'anime help' to see the available commands");
    process::exit(1);
}
pub fn add_ani(connec: &Connection, ani: &mut Anime) -> ResultSql<()> {
    let insert_ani = db::add_data(&connec, ani)?;
    if insert_ani {
        db::show_db(&connec)
    } else {
        println!("Anime already exists in your database: {}", ani.nome);
        Ok(())
    }
}
pub fn get_input_eps() -> u32 {
    println!("Enter the number of epsodes seen: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error enter a valide number");
    let num_seen: u32 = input.trim().parse().expect("Erro ao converter");
    num_seen
}
pub fn check_rm(r: bool, connec: &Connection, ani_nome: &str) -> ResultSql<()> {
    if r {
        println!("Anime '{}' removed successfully", ani_nome);
        db::show_db(&connec)?;
        Ok(())
    } else {
        println!("Error: unable to remove the anime.");
        Ok(())
    }
}
pub fn check_db_path() -> bool {
    let home_dir = env::var("HOME").expect("Error: directory home not found");
    let db_dir = Path::new(&home_dir).join(".anime_cli").join("database.db");
    if db_dir.exists() { true } else { false }
}
