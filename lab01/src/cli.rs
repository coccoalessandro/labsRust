
pub struct Config {
    pub filename: String,
    pub head: usize,
}

pub fn parse_args(args: &[String]) -> Result<Config, String> {

    if args.len() < 2 || args.len() > 3 {
        return Err("Errore nell'inserimento dell'input".to_string());
    }

    let filename = args[1].clone();

    let head: usize;

    if args.len() == 3 {
        head = match args[2].parse::<usize>() {
            Err(_) => return Err("Errore nell'inserimeto dell'input".to_string()),
            Ok(head) => head,
        };
    }
    else {
        head = 10;
    }

    Ok(Config {filename, head})
}