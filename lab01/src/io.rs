use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(path: &str, head: usize) {
    
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut count_lines = 0;

    println!("head ({}):", head);

    for line in reader.lines() {
        count_lines = count_lines + 1;
        let riga = line.unwrap();
        if count_lines <= head {
            println!("{}", riga);
        }
    }
    
    println!("rows: {}", count_lines);
}