mod cli;
mod db;
use cli::{HELP_TEXT, add_ani, check_rm, get_input_eps, print_err};
use db::Anime;
use rusqlite::Result as ResultSql;
use std::env;

use crate::cli::{COMMUN_PATH, check_db_path};
/* cli file: functions related to cli in memory insted db
 * db file: functions related to database and the struct Anime
 * main file: functions related to read args and control flow
 */
fn main() -> ResultSql<()> {
    let args: Vec<String> = env::args().collect();
    let connec = db::new_db()?;
    db::init_db(&connec)?;
    match args.len() {
        4 => {
            let opt: &str = &args[1];
            let anime_name: &str = &args[2];
            let num_ep: u32 = args[3].parse().unwrap_or(0);
            match opt {
                "add" => {
                    let mut ani = Anime::new(anime_name, num_ep);
                    add_ani(&connec, &mut ani)?;
                }
                _ => print_err(),
            }
        }
        3 => {
            let opt: &str = &args[1];
            let anime_name: &str = &args[2];
            match db::search_ani(&connec, anime_name) {
                Ok(mut anime) => match opt {
                    "up" => {
                        let num_seen: u32 = get_input_eps();
                        db::update_ep(&connec, &mut anime, num_seen)?;
                        db::show_db(&connec)?;
                    }
                    "rm" => {
                        let deleted_item = db::remove_ani(&connec, &anime)?;
                        //if else statement in cli.rs 46 ->
                        check_rm(deleted_item, &connec, &anime.nome)?;
                    }
                    _ => print_err(),
                },
                Err(_) => {
                    println!(
                        "Anime not found in database, make sure you entered the correct name or id"
                    );
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

                "-Sd" => {
                    if check_db_path() {
                        println!("Path to dabase.db: {}", COMMUN_PATH);
                    } else {
                        println!("Error {} not found", COMMUN_PATH);
                    }
                }
                "-h" => {
                    println!("{}", HELP_TEXT);
                }
                "help" => {
                    println!("{}", HELP_TEXT);
                }
                _ => print_err(),
            }
        }
        _ => print_err(),
    }
    Ok(())
}
