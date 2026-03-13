mod cli;
mod io;

use std::env::args;
use std::process::exit;

fn main() {

    let args: Vec<String> = args().collect();
    
    let config = match cli::parse_args(&args) {
        Ok(config) => config,
        Err(e) => {eprintln!("{}", e); exit(1);}
    };

    match io::read_file(&config.filename, config.head) {
        Ok(_) => {},
        Err(e) => {eprintln!("{}", e); exit(1);}
    };
}