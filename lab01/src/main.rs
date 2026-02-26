mod cli;
mod io;

use std::env::args;

fn main() {

    let args: Vec<String> = args().collect();
    
    let config = cli::parse_args(&args);

    io::read_file(&config.filename, config.head);
}