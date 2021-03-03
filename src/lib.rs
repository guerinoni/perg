use std::{
    fs,
    io::{self, BufRead},
    path,
};

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

pub struct Config<'a> {
    pattern: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(pattern: &'a str, filename: &'a str) -> Config<'a> {
        Config { pattern, filename }
    }
}

pub fn grep(c: Config) -> Result<Vec<String>, &'static str> {
    let path = path::Path::new(c.filename);
    if !path.exists() {
        println!("No such file or directory");
        return Err("No such file or directory");
    }

    if path.is_file() {
        let file = fs::File::open(path).unwrap();
        let lines = io::BufReader::new(file).lines();
        let items: Vec<String> = lines
            .par_bridge()
            .into_par_iter()
            .filter_map(|i| i.ok())
            .filter(|i| i.contains(c.pattern))
            .collect();

        return Ok(items);
    }

    // WalkDir::new(c.filename)
    //     .into_iter()
    //     .filter_entry(|e| !is_hidden(e))
    //     .par_bridge()
    //     .into_par_iter()
    //     .filter_map(|rd| rd.ok())
    //     .filter(|e| e.metadata().unwrap().is_file())
    //     .for_each(|e| {
    //         let file = fs::File::open(e.path()).unwrap();
    //         let lines = io::BufReader::new(file).lines();
    //         lines
    //             .par_bridge()
    //             .into_par_iter()
    //             .filter_map(|i| i.ok())
    //             .filter(|i| i.contains(c.want_search))
    //             .for_each(|item| println!("{}: {}", e.path().display(), item));
    //     })

    Ok(Vec::new())
}

// fn is_hidden(entry: &walkdir::DirEntry) -> bool {
//     entry
//         .file_name()
//         .to_str()
//         .map(|s| s.starts_with('.') && s.len() > 1)
//         .unwrap_or(false)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_path_invalid() {
        let c = Config::new("hello", "/home/invalid");
        let r = grep(c);
        assert_eq!(r, Err("No such file or directory"));
    }

    #[test]
    fn grep_single_file() {
        let c = Config::new("federico", "./Cargo.toml");
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![String::from(
                "authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"
            )])
        );
    }
}
