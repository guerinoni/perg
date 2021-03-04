use std::{
    fs,
    io::{self, BufRead},
    path,
};

pub struct Config<'a> {
    pattern: &'a str,
    filename: &'a str,
    line_number: bool,
}

impl<'a> Config<'a> {
    pub fn new(pattern: &'a str, filename: &'a str, line_number: bool) -> Config<'a> {
        Config {
            pattern,
            filename,
            line_number,
        }
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
        let mut items = Vec::new();
        for (idx, str) in lines.enumerate() {
            if let Ok(item) = str {
                if item.contains(c.pattern) {
                    let mut s = String::from("");
                    if c.line_number {
                        s = format!("{}: ", idx + 1);
                    }

                    s.push_str(item.as_str());
                    items.push(s);
                }
            }
        }
        // .par_bridge()
        // .into_par_iter()
        // .filter(|i| i.1.ok().is_some() && i.1.unwrap().contains(c.pattern))
        // .collect();

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
        let c = Config::new("hello", "/home/invalid", false);
        let r = grep(c);
        assert_eq!(r, Err("No such file or directory"));
    }

    #[test]
    fn grep_single_file() {
        let c = Config::new("federico", "./Cargo.toml", false);
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![String::from(
                "authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"
            )])
        );
    }

    #[test]
    fn grep_single_file_with_line_number() {
        let c = Config::new("federico", "./Cargo.toml", true);
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![String::from(
                "4: authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"
            )])
        );
    }
}
