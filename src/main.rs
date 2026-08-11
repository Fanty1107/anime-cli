use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    
    println!("{}", argv[0]);

    if let Some(arg1) = argv.get(1){
        println!("{}", arg1);
        
    }
    else{
        println!("no args passed");
    }
}
