mod cli;
mod io;

use std::env::args;

fn main() {

    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("Errore. Devi definire il nome del file da leggere.");
        std::process::exit(1);
    }

    let path = &args[1];

    io::read_file(path);
}