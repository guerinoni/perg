use std::{
    fs,
    io::{self, BufRead, Read},
    path,
};

pub struct Config<'a> {
    pattern: &'a str,
    filenames: Vec<&'a str>,
    line_number: bool,
}

impl<'a> Config<'a> {
    pub fn new(pattern: &'a str, filenames: Vec<&'a str>, line_number: bool) -> Config<'a> {
        Config {
            pattern,
            filenames,
            line_number,
        }
    }
}

pub fn grep(c: Config) -> Result<Vec<String>, &'static str> {
    if c.pattern == "-" {
        let mut buffer = String::new();
        let r = io::stdin().read_to_string(&mut buffer).unwrap_or_default();
        dbg!(r);
    }

    if c.filenames.len() == 1 && c.filenames[0] == "-" {
        dbg!("read from stdin");
    }

    let mut items = Vec::new();
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

    #[test]
    fn return_path_invalid() {
        let c = Config::new("hello", vec!["/home/invalid"], false);
        let r = grep(c);
        assert_eq!(r, Err("No such file or directory"));
    }

    #[test]
    fn grep_single_file() {
        let c = Config::new("federico", vec!["./Cargo.toml"], false);
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
        let c = Config::new("federico", vec!["./Cargo.toml", "./Cargo.toml"], false);
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
        let c = Config::new("federico", vec!["./Cargo.toml"], true);
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![String::from(
                "4: authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"
            )])
        );
    }
}
