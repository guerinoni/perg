use std::{
    fs,
    io::{self, BufRead},
    path,
};

use walkdir::WalkDir;

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

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s.len() > 1)
        .unwrap_or(false)
}

pub fn grep(c: Config) -> Vec<String> {
    let mut results = Vec::new();

    let path = path::Path::new(c.filename);
    if !path.exists() {
        return results;
    }

    if path::Path::is_file(path) {
        let r = grep_single_file(path.to_str().unwrap(), c.want_search);
        results.extend(r);
    }

    if path::Path::is_dir(path) {
        for e in WalkDir::new(path)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter(|e| e.is_ok())
            .filter_map(|e| e.ok())
        {
            if e.metadata().unwrap().is_file() {
                let r = grep_single_file(e.path().to_str().unwrap(), c.want_search);
                results.extend(r);
            }
        }
    }

    results
}

fn grep_single_file(filename: &str, to_search: &str) -> Vec<String> {
    let mut results = Vec::new();
    let file = fs::File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    for l in lines {
        if let Ok(line) = l {
            if line.contains(to_search) {
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
            filename: "Cargo.toml",
        };

        let res = grep(c);
        assert_eq!(
            res,
            vec!["authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"]
        );
    }

    #[test]
    fn grep_current_subfolder() {
        let c = Config {
            want_search: "perg",
            filename: "./src",
        };

        assert_eq!(grep(c).len(), 2);
    }

    #[test]
    fn grep_current_folder() {
        let c = Config {
            want_search: "readme",
            filename: ".",
        };

        assert_eq!(grep(c).len(), 3);
    }
}
