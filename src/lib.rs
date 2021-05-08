use jwalk::WalkDir;
use std::{
    fs,
    io::{self, BufRead, Read},
    path,
};

pub struct Config<'a> {
    pattern: &'a str,
    filenames: Vec<&'a str>,
    line_number: bool,
    recursive: bool,
}

impl<'a> Config<'a> {
    pub fn new(
        pattern: &'a str,
        filenames: Vec<&'a str>,
        line_number: bool,
        recursive: bool,
    ) -> Config<'a> {
        Config {
            pattern,
            filenames,
            line_number,
            recursive,
        }
    }
}

pub fn grep(c: Config) -> Result<Vec<String>, &'static str> {
    let mut items = Vec::new();
    if c.recursive {
        for entry in WalkDir::new(c.filenames.get(0).unwrap()).skip_hidden(true) {
            let entry = entry.unwrap();
            if !entry.file_type().is_file() {
                continue;
            }

            let file = fs::File::open(entry.path()).expect("can't open file");
            let lines = io::BufReader::new(file).lines();
            for (idx, str) in lines.enumerate() {
                if let Ok(item) = str {
                    if item.contains(c.pattern) {
                        let mut s = String::from("");
                        s = format!("{:?}: ", entry.path().to_str().unwrap());
                        if c.line_number {
                            s = format!("{}{}: ", s, idx + 1);
                        }

                        s.push_str(item.as_str());
                        items.push(s);
                    }
                }
            }
        }
    }

    if c.pattern == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap_or_default();
    }

    if c.filenames.is_empty() || c.filenames.len() == 1 && c.filenames[0] == "-" {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let item = line.unwrap_or_default();
            if item.contains(c.pattern) {
                println!("{}", item);
            }
        }
    }

    for path in c.filenames {
        let path = path::Path::new(path);
        if !path.exists() {
            println!("No such file or directory");
            return Err("No such file or directory");
        }

        if path.is_file() {
            let file = fs::File::open(path).unwrap();
            let lines = io::BufReader::new(file).lines();
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
        }
    }

    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn return_path_invalid() {
        let c = Config::new("hello", vec!["/home/invalid"], false, false);
        let r = grep(c);
        assert_eq!(r, Err("No such file or directory"));
    }

    #[test]
    fn grep_single_file() {
        let c = Config::new("federico", vec!["./Cargo.toml"], false, false);
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![String::from(
                "authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"
            )])
        );
    }

    #[test]
    fn grep_two_file() {
        let c = Config::new(
            "federico",
            vec!["./Cargo.toml", "./Cargo.toml"],
            false,
            false,
        );
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![
                String::from("authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"),
                String::from("authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]")
            ])
        );
    }

    #[test]
    fn grep_single_file_with_line_number() {
        let c = Config::new("federico", vec!["./Cargo.toml"], true, false);
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![String::from(
                "4: authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"
            )])
        )
    }

    #[test]
    fn grep_folder() {
        let c = Config::new("nulla", vec!["./testdata"], false, true);
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![
                String::from("\"./testdata/lol\": Suspendisse potenti. Curabitur vestibulum varius tellus, ut feugiat nulla ornare quis. "),
                String::from("\"./testdata/lol\": Aenean aliquam lacus ex, in gravida est mollis at. Etiam consectetur luctus nulla eu porttitor. "),
                String::from("\"./testdata/lol\": Aliquam pharetra nulla placerat interdum laoreet. Vestibulum facilisis metus eu erat suscipit malesuada. "),
                String::from("\"./testdata/folder/lol\": Suspendisse potenti. Curabitur vestibulum varius tellus, ut feugiat nulla ornare quis. "),
                String::from("\"./testdata/folder/lol\": Aenean aliquam lacus ex, in gravida est mollis at. Etiam consectetur luctus nulla eu porttitor. "),
                String::from("\"./testdata/folder/lol\": Aliquam pharetra nulla placerat interdum laoreet. Vestibulum facilisis metus eu erat suscipit malesuada. "),
        ])
        );
    }
}
