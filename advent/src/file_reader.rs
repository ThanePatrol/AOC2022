use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

//gets line separated values
pub fn get_lines_separated(path: &str) -> Vec<Vec<String>> {
    let f = File::open(path).expect("");
    let mut file = BufReader::new(f);

    let mut entries = Vec::new();

    let mut iter = file.lines();

    let mut entry = Vec::new();
    for line in iter {
        let val = line.unwrap();
        if val == "" {
            entries.push(entry.clone());
            let new = Vec::new();
            entry = new;
        } else {
            entry.push(val.parse().unwrap())
        }
    }

    entries
}

pub fn get_lines_as_vec(path: &str) -> Vec<String> {
    let f = File::open(path).expect("");
    let mut file = BufReader::new(f);
    let mut entries = Vec::new();
    for line in file.lines() {
        entries.push(line.unwrap())
    }
    entries
}