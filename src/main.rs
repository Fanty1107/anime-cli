mod db;
use std::{env, io, process};
use rusqlite::Result as ResultSql;

use db::{Anime};

const HELP_TEXT: &str = "\
Usage: anime-cli <COMMAND> <ANIME_NAME> [ARGS]

Commands:
  add <name> <total_eps>   Adds a new anime to the database.
  up <name>                Updates the watched episodes for a specific anime.
  rm <name>                Removes an anime from the database.
  show/-s                  Show the local database. 

Examples:
  anime-cli add \"One Piece\" 1070
  *****     up \"Naruto\"
  *****     up \"id_anime\"  
  *****     rm \"Bleach\"
  *****     rm \"id_anime\" 
  *****     show/-s
  ";

fn main()-> ResultSql<()>{
    let args: Vec<String> = env::args().collect();
    let connec = db::new_db()?;
    db::init_db(&connec)?; 
    match args.len() {
        4 => {
            let opt: &str = &args[1];
            let anime_name: &str = &args[2];
            let num_ep: u32 = args[3].parse().unwrap_or(0);
            match opt{
                "add" =>{
                    let mut ani = Anime::new(anime_name, num_ep);
                    let insert_ani = db::add_data(&connec, &mut ani)?;
                    if insert_ani{
                        db::show_db(&connec)?;
                    }else {
                        println!("Anime already exists in your database: {}", ani.nome);
                    }
                },
                _ => {
                    eprintln!("Error: command unknow");
                    eprintln!("Use 'anime help' to see the available commands");
                    process::exit(1);
                }
            }
        },
        3 =>{
            let opt: &str = &args[1];
            let anime_name: &str = &args[2];
            match db::search_ani(&connec, anime_name) {
               Ok(mut anime) => match opt {
                   "up" =>{
                        println!("Enter the number of epsodes seen: ");
                        let mut input = String::new();
                        io::stdin()
                        .read_line(&mut input)
                        .expect("Error enter a valide number");

                        let num_seen: u32 = input.trim().parse().expect("Erro ao converter");
                        db::update_ep(&connec, &mut anime, num_seen)?;
                        db::show_db(&connec)?; 
                   },
                   "rm" =>{
                        let deleted_item = db::remove_ani(&connec, &anime)?;
                        if deleted_item{
                            println!("Anime '{}' removed successfully", anime.nome);
                            db::show_db(&connec)?;
                        }
                        else{
                            println!("Error: unable to remove the anime.");
                        }
                   },
                   _ => {
                    eprintln!("Error: command unknow");
                    eprintln!("Use 'anime help' to see the available commands");
                    process::exit(1);
                }
               }
               Err(_) =>{
                println!("Anime not found in database, make sure you entered the correct name or id");
               }
            }
        }
        2 => {
            let opt: &str = &args[1];
            match opt {
                "show" => {
                    db::show_db(&connec)?;
                }
                "-s" => {
                    db::show_db(&connec)?;
                }
                "-h" =>{
                    println!("{}", HELP_TEXT);
                }
                "help" =>{
                    println!("{}", HELP_TEXT);
                }
                _ => {
                    eprintln!("Error: command unknow");
                    eprintln!("Use 'anime help' to see the available commands");
                    process::exit(1);
                }
            }
        }
        _ => {
                eprintln!("Error: command unknow");
                eprintln!("Use 'anime help' to see the available commands");
                process::exit(1);
        }
    }
    Ok(())
}
