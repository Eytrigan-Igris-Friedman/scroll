use std::env;
use std::io::{self, Write};
use std::process;



fn run_repl() {
    let mut line_buffer = String::new();
    
   loop {
        print!(">> ");
        let _ = io::stdout().flush();
        
        match io::stdin().read_line(&mut line_buffer) {
        Ok(_) => {
            println!("{}", line_buffer);
            line_buffer.clear();
            ()
        },
        Err(_) => {
            eprintln!("Error could not load file");
            ()
        }
    }
   }

}



fn file_exec(_file_path: String){
    todo!();
}




fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => {
            run_repl();
        },
        2 => {
            file_exec(args[1].to_string());
        },
        _ => (),
    }
}
