
pub struct Config {
    pub filename: String,
    pub head: usize,
}

pub fn parse_args(args: &[String]) -> Config {

    if args.len() < 2 {
        eprintln!("Errore. Devi definire il nome del file da leggere.");
        std::process::exit(1);
    }

    let filename = args[1].clone();

    let head: usize;

    if args.len() == 3 {
        head = args[2].parse::<usize>().unwrap();
    }
    else {
        head = 10;
    }

    Config {filename, head}
}