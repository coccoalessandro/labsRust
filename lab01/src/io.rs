use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(path: &str) {
    
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut count_lines = 0;

    for _line in reader.lines() {
        count_lines = count_lines + 1;
    }

    println!("rows: {}", count_lines);
}