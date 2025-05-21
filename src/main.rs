use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => println!("No argment passed"),
        2 => println!("File: {}", args[1]),
        _ => (),
    }
}
