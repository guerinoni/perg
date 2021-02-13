use std::{fs, io};

use io::BufRead;

pub struct Config<'a> {
    want_search: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(want_search: &'a str, filename: &'a str) -> Config<'a> {
        Config {
            want_search,
            filename,
        }
    }
}

pub fn grep(c: Config) -> Vec<String> {
    let file = fs::File::open(c.filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut results = Vec::new();
    for l in lines {
        if let Ok(line) = l {
            if line.contains(c.want_search) {
                results.push(line);
            }
        }
    }

    results
}
