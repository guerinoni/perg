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
        Config {
            pattern,
            filename,
        }
    }
}

pub fn grep(c: Config) {
    let path = path::Path::new(c.filename);
    if !path.exists() {
        println!("No such file or directory");
        return;
    }

    if path.is_file() {
        let file = fs::File::open(path).unwrap();
        let lines = io::BufReader::new(file).lines();
        lines
            .par_bridge()
            .into_par_iter()
            .filter_map(|i| i.ok())
            .filter(|i| i.contains(c.pattern))
            .for_each(|item| println!("{}: {}", path.display(), item));
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
}

// fn is_hidden(entry: &walkdir::DirEntry) -> bool {
//     entry
//         .file_name()
//         .to_str()
//         .map(|s| s.starts_with('.') && s.len() > 1)
//         .unwrap_or(false)
// }
