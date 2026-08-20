pub mod cli;
pub mod db;

pub use cli::{COMMUN_PATH, HELP_TEXT, add_ani, check_db_path, check_rm, get_input_eps, print_err};
pub use db::{add_data, init_db, new_db, remove_ani, search_ani, show_db, update_ep};

pub struct Anime {
    pub id: Option<u32>,
    pub nome: String,
    pub num_ep: u32,
    pub cur_ep: u32,
}

impl Anime {
    pub fn new(n: &str, n_ep: u32) -> Self {
        Self {
            id: None,
            nome: n.to_string(),
            num_ep: n_ep,
            cur_ep: 0,
        }
    }
}
