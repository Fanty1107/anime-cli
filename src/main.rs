use anime_cli::{
    Anime, COMMUN_PATH, HELP_TEXT, add_ani, check_db_path, check_rm, get_input_eps, init_db,
    new_db, print_err, remove_ani, search_ani, show_db, update_ep,
};
use rusqlite::Result as ResultSql;
use std::env;

/* cli file: functions related to cli in memory insted db
 * db file: functions related to database and the struct Anime
 * main file: functions related to read args and control flow
 *
 *
 * TODO: Option for outdated a current_ep, and max Current_ep with Episodes with max or flag -M
 */
fn main() -> ResultSql<()> {
    let args: Vec<String> = env::args().collect();
    let connec = new_db()?;
    init_db(&connec)?;
    match args.len() {
        4 => {
            let (opt, anime_name, num_ep): (&str, &str, u32) =
                (&args[1], &args[2], args[3].parse().unwrap_or(0));
            match opt {
                "add" => {
                    let mut ani = Anime::new(anime_name, num_ep);
                    add_ani(&connec, &mut ani)?;
                }
                "up" => {
                    let mut anime_up = search_ani(&connec, anime_name).unwrap();
                    update_ep(&connec, &mut anime_up, num_ep)?;
                    show_db(&connec)?;
                }
                _ => print_err(),
            }
        }
        3 => {
            let opt: &str = &args[1];
            let anime_name: &str = &args[2];
            match search_ani(&connec, anime_name) {
                Ok(mut anime) => match opt {
                    "up" => {
                        let num_seen: u32 = get_input_eps();
                        update_ep(&connec, &mut anime, num_seen)?;
                        show_db(&connec)?;
                    }
                    "rm" => {
                        let deleted_item = remove_ani(&connec, &anime)?;
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
                    show_db(&connec)?;
                }
                "-s" => {
                    show_db(&connec)?;
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
