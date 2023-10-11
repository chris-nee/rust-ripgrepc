use std::env;

fn main() {
    println!("Welcome to ripgrepc!");
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("At least 2 arguments required, FILE PATH, SEARCH TEXT");
        panic!()
    }

    println!("Printing args: ");
    for i in 0..args.len() {
        println!("-> [{}]", args[i]);
    }

    dbg!();
}
