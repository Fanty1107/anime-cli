use core::fmt;
use std::env;

struct Anime{
    nome: String,
    num_ep: u32,
}
impl Anime {
    fn new(n: String, n_ep: u32)-> Self {
       Self{nome: n, num_ep: n_ep} 
    }
    fn get_num_eps(&self) -> u32{
        15
    }
}
impl fmt::Display for Anime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Anime: {}, Eps: {}", self.nome, self.num_ep)
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    
    println!("{}", argv[0]);

    if let Some(arg1) = argv.get(1){
        println!("{}", arg1);
       let ani = Anime::new(arg1.to_string(), 12);
       ani.get_num_eps();
       println!("{ani}");
    }
    else{
        println!("no args passed");
    }
}
