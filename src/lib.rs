use std::{
    fs,
    io::{self, BufRead},
    path,
};

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

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
    let path = path::Path::new(c.filename);
    if !path.exists() {
        return Vec::new();
    }

    if path::Path::is_file(path) {
        let r = grep_single_file(path.to_str().unwrap(), c.want_search);
        let mut results = Vec::new();
        for i in r {
            results.push(i.unwrap());
        }
        return results;
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
                let mut results = Vec::new();
                for i in r {
                    results.push(i.unwrap());
                }
                dbg!(results.clone());
                return results;
            }
        }
    }

    Vec::new()
}

fn grep_single_file(filename: &str, to_search: &str) -> Vec<Result<String, io::Error>> {
    let file = fs::File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .par_bridge()
        .into_par_iter()
        .filter(|l| l.is_ok() && l.as_ref().unwrap().contains(to_search))
        .collect::<Vec<_>>()
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
            want_search: "readme",
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

        assert_eq!(grep(c).len(), 2);
    }
}
