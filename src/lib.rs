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
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn new(
        pattern: &'a str,
        filenames: Vec<&'a str>,
        line_number: bool,
        recursive: bool,
        ignore_case: bool,
    ) -> Config<'a> {
        Config {
            pattern,
            filenames,
            line_number,
            recursive,
            ignore_case,
        }
    }
}

fn search_in_file(
    filename: &str,
    pattern: &'_ str,
    show_line_number: bool,
    show_filename: bool,
    ignore_case: bool,
) -> Vec<String> {
    let mut items = Vec::new();
    let path = path::Path::new(filename);
    if !path.is_file() {
        return items;
    }

    let file = fs::File::open(filename).expect("can't open file");
    let lines = io::BufReader::new(file).lines();
    for (idx, str) in lines.enumerate() {
        if let Ok(item) = str {
            let mut item_ = item.clone();
            let mut pattern = pattern.to_string();
            if ignore_case {
                item_ = item_.to_lowercase();
                pattern = pattern.to_lowercase();
            }

            if item_.contains(pattern.as_str()) {
                let mut s = String::from("");
                if show_filename {
                    s = format!("{:?}: ", filename);
                }
                if show_line_number {
                    s = format!("{}{}: ", s, idx + 1);
                }

                s.push_str(item.as_str());
                items.push(s);
            }
        }
    }

    items
}

pub fn grep(c: Config) -> Result<Vec<String>, &'static str> {
    let mut items = Vec::new();
    if c.recursive {
        for entry in WalkDir::new(c.filenames.get(0).unwrap()).skip_hidden(true) {
            let entry = entry.unwrap();
            let mut res = search_in_file(
                entry.path().to_str().unwrap(),
                c.pattern,
                c.line_number,
                true,
                c.ignore_case,
            );
            items.append(&mut res);
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
            let mut item_ = item.clone();
            let mut pattern = c.pattern.to_string();
            if c.ignore_case {
                item_ = item_.to_lowercase();
                pattern = pattern.to_lowercase();
            }

            if item_.contains(pattern.as_str()) {
                println!("{}", item);
            }
        }
    }

    for filename in c.filenames {
        let path = path::Path::new(filename);
        if !path.exists() {
            println!("No such file or directory");
            return Err("No such file or directory");
        }

        let mut res = search_in_file(filename, c.pattern, c.line_number, false, c.ignore_case);
        items.append(&mut res);
    }

    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn return_path_invalid() {
        let c = Config::new("hello", vec!["/home/invalid"], false, false, false);
        let r = grep(c);
        assert_eq!(r, Err("No such file or directory"));
    }

    #[test]
    fn grep_single_file() {
        let c = Config::new("federico", vec!["./Cargo.toml"], false, false, false);
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![String::from(
                "authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"
            )])
        );

        let c = Config::new(
            "federico guerinoni",
            vec!["./Cargo.toml"],
            false,
            false,
            false,
        );
        let r = grep(c);
        assert_eq!(r, Ok(vec![]));
    }

    #[test]
    fn grep_single_file_ignore_case() {
        let c = Config::new(
            "federico guerinoni",
            vec!["./Cargo.toml"],
            false,
            false,
            true,
        );
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
        let c = Config::new("federico", vec!["./Cargo.toml"], true, false, false);
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![String::from(
                "4: authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"
            )])
        )
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn grep_folder() {
        let c = Config::new("nulla", vec!["./testdata"], false, true, false);
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

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn grep_folder_ignore_case() {
        let c = Config::new("nulla", vec!["./testdata"], false, true, true);
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![
                String::from("\"./testdata/lol\": Suspendisse potenti. Curabitur vestibulum varius tellus, ut feugiat nulla ornare quis. "),
                String::from("\"./testdata/lol\": Nulla tincidunt purus et semper suscipit. Donec porta ex at elit cursus, eget tristique mi fermentum."),
                String::from("\"./testdata/lol\": Aenean aliquam lacus ex, in gravida est mollis at. Etiam consectetur luctus nulla eu porttitor. "),
                String::from("\"./testdata/lol\": Nullam vel sollicitudin dui, sit amet tincidunt nibh. Sed pretium sem a ipsum tincidunt posuere. "),
                String::from("\"./testdata/lol\": Morbi venenatis ex mauris, tincidunt aliquet magna pharetra vehicula. Nullam lacinia nec velit eget pharetra. "),
                String::from("\"./testdata/lol\": Nullam sagittis faucibus varius."),
                String::from("\"./testdata/lol\": Aliquam pharetra nulla placerat interdum laoreet. Vestibulum facilisis metus eu erat suscipit malesuada. "),
                String::from("\"./testdata/lol\": Nulla hendrerit felis a mauris mollis mollis id nec mi. Etiam sit amet fringilla diam, a maximus urna."),
                String::from("\"./testdata/folder/lol\": Suspendisse potenti. Curabitur vestibulum varius tellus, ut feugiat nulla ornare quis. "),
                String::from("\"./testdata/folder/lol\": Nulla tincidunt purus et semper suscipit. Donec porta ex at elit cursus, eget tristique mi fermentum."),
                String::from("\"./testdata/folder/lol\": Aenean aliquam lacus ex, in gravida est mollis at. Etiam consectetur luctus nulla eu porttitor. "),
                String::from("\"./testdata/folder/lol\": Nullam vel sollicitudin dui, sit amet tincidunt nibh. Sed pretium sem a ipsum tincidunt posuere. "),
                String::from("\"./testdata/folder/lol\": Morbi venenatis ex mauris, tincidunt aliquet magna pharetra vehicula. Nullam lacinia nec velit eget pharetra. "),
                String::from("\"./testdata/folder/lol\": Nullam sagittis faucibus varius.",),
                String::from("\"./testdata/folder/lol\": Aliquam pharetra nulla placerat interdum laoreet. Vestibulum facilisis metus eu erat suscipit malesuada. "),
                String::from("\"./testdata/folder/lol\": Nulla hendrerit felis a mauris mollis mollis id nec mi. Etiam sit amet fringilla diam, a maximus urna."),
        ])
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn grep_folder_with_line_numbers() {
        let c = Config::new("nulla", vec!["./testdata"], true, true, false);
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![
                String::from("\"./testdata/lol\": 12: Suspendisse potenti. Curabitur vestibulum varius tellus, ut feugiat nulla ornare quis. "),
                String::from("\"./testdata/lol\": 23: Aenean aliquam lacus ex, in gravida est mollis at. Etiam consectetur luctus nulla eu porttitor. "),
                String::from("\"./testdata/lol\": 39: Aliquam pharetra nulla placerat interdum laoreet. Vestibulum facilisis metus eu erat suscipit malesuada. "),
                String::from("\"./testdata/folder/lol\": 12: Suspendisse potenti. Curabitur vestibulum varius tellus, ut feugiat nulla ornare quis. "),
                String::from("\"./testdata/folder/lol\": 23: Aenean aliquam lacus ex, in gravida est mollis at. Etiam consectetur luctus nulla eu porttitor. "),
                String::from("\"./testdata/folder/lol\": 39: Aliquam pharetra nulla placerat interdum laoreet. Vestibulum facilisis metus eu erat suscipit malesuada. "),
        ])
        );
    }
}
