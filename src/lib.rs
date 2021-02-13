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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn grep_single_file() {
        let c = Config {
            want_search: "federico",
            filename: "Cargo.toml"
        };

        let res = grep(c);
        assert_eq!(res, vec!["authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"]);
    }
}