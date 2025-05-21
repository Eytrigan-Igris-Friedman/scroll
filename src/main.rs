use std::env;
use std::io::{self, Write};
use std::process;

mod terminal_raw_mode;
use terminal_raw_mode::enable_raw_mode;
use terminal_raw_mode::disable_raw_mode;

fn run_repl() {
    // let mut line_buffer = String::new();
    let mut buffer: &[u8] = &[0; 1];
    
   loop {
        print!(">> ");
        let _ = io::stdout().flush();
        
        enable_raw_mode();
        
        match io::stdin().read_exact(&mut buffer) {
        Ok(_) => {
            let newline = buffer[0] as char;
            
            if newline == '\n' {
                disable_raw_mode();
                process::exit(0)
            }
            println!("{}", buffer);
            buffer.clear();
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
