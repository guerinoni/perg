use core::time;
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
    recursive_following_symlink: bool,
    ignore_case: bool,
    exclude_dir: Option<&'a str>,
}

impl<'a> Config<'a> {
    pub fn new(
        pattern: &'a str,
        filenames: Vec<&'a str>,
        line_number: bool,
        recursive: bool,
        recursive_following_symlink: bool,
        ignore_case: bool,
        exclude_dir: Option<&'a str>,
    ) -> Config<'a> {
        Config {
            pattern,
            filenames,
            line_number,
            recursive,
            recursive_following_symlink,
            ignore_case,
            exclude_dir,
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

    let file = match fs::File::open(filename) {
        Ok(f) => f,
        Err(_) => return vec![],
    };
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
                    s = format!("{}:", filename);
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

pub fn grep(mut c: Config) -> Result<Vec<String>, &'static str> {
    let mut items = Vec::new();
    if c.recursive || c.recursive_following_symlink {
        if c.filenames.is_empty() {
            c.filenames = vec!["./"];
        }
        for entry in WalkDir::new(c.filenames.first().unwrap())
            .skip_hidden(true)
            .parallelism(jwalk::Parallelism::RayonDefaultPool {
                busy_timeout: time::Duration::from_secs(1),
            })
        {
            let entry = entry.unwrap();
            if !c.recursive_following_symlink && entry.path_is_symlink() {
                continue;
            }
            let path = entry.path();
            let p = path.to_str().unwrap();
            if let Some(exclude) = c.exclude_dir
                && path
                    .parent()
                    .unwrap()
                    .ancestors()
                    .any(|p| p.ends_with(exclude))
            {
                continue;
            }
            let mut res = search_in_file(p, c.pattern, c.line_number, true, c.ignore_case);
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
        let c = Config::new(
            "hello",
            vec!["/home/invalid"],
            false,
            false,
            false,
            false,
            None,
        );
        let r = grep(c);
        assert_eq!(r, Err("No such file or directory"));
    }

    #[test]
    fn grep_single_file() {
        let c = Config::new(
            "federico",
            vec!["./Cargo.toml"],
            false,
            false,
            false,
            false,
            None,
        );
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
            false,
            None,
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
            false,
            true,
            None,
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
            false,
            None,
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
        let c = Config::new(
            "federico",
            vec!["./Cargo.toml"],
            true,
            false,
            false,
            false,
            None,
        );
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
        let c = Config::new("you", vec!["./testdata"], false, true, false, false, None);
        let mut r = grep(c).unwrap();
        r.sort();
        assert_eq!(
            r,
            vec![
                "./testdata/folder/lol:Evening green fill you'll gathering above hath.",
                "./testdata/folder/lol:He divide for appear deep abundantly. Had above unto. Moving stars fish. Whose you'll can't beginning sixth.",
                "./testdata/folder/lol:Third dominion you're had called green.",
                "./testdata/folder/lol:Tree brought multiply land darkness had dry you're of.",
                "./testdata/lol:Evening green fill you'll gathering above hath.",
                "./testdata/lol:He divide for appear deep abundantly. Had above unto. Moving stars fish. Whose you'll can't beginning sixth.",
                "./testdata/lol:Third dominion you're had called green.",
                "./testdata/lol:Tree brought multiply land darkness had dry you're of."
            ]
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn grep_folder_ignore_case() {
        let c = Config::new("you", vec!["./testdata"], false, true, false, true, None);
        let mut r = grep(c).unwrap();
        r.sort();
        assert_eq!(
            r,
            vec![
                "./testdata/folder/lol:Evening green fill you'll gathering above hath.",
                "./testdata/folder/lol:He divide for appear deep abundantly. Had above unto. Moving stars fish. Whose you'll can't beginning sixth.",
                "./testdata/folder/lol:Third dominion you're had called green.",
                "./testdata/folder/lol:Tree brought multiply land darkness had dry you're of.",
                "./testdata/folder/lol:You.",
                "./testdata/lol:Evening green fill you'll gathering above hath.",
                "./testdata/lol:He divide for appear deep abundantly. Had above unto. Moving stars fish. Whose you'll can't beginning sixth.",
                "./testdata/lol:Third dominion you're had called green.",
                "./testdata/lol:Tree brought multiply land darkness had dry you're of.",
                "./testdata/lol:You."
            ]
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn grep_folder_without_specify_folder() {
        let c = Config::new("you", vec![], false, true, false, false, None);
        let r = grep(c);
        assert!(r.is_ok());
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn grep_folder_with_line_numbers() {
        let c = Config::new("you", vec!["./testdata"], true, true, false, false, None);
        let mut r = grep(c).unwrap();
        r.sort();
        assert_eq!(
            r,
            vec![
                "./testdata/folder/lol:19: He divide for appear deep abundantly. Had above unto. Moving stars fish. Whose you'll can't beginning sixth.",
                "./testdata/folder/lol:25: Third dominion you're had called green.",
                "./testdata/folder/lol:26: Evening green fill you'll gathering above hath.",
                "./testdata/folder/lol:8: Tree brought multiply land darkness had dry you're of.",
                "./testdata/lol:19: He divide for appear deep abundantly. Had above unto. Moving stars fish. Whose you'll can't beginning sixth.",
                "./testdata/lol:25: Third dominion you're had called green.",
                "./testdata/lol:26: Evening green fill you'll gathering above hath.",
                "./testdata/lol:8: Tree brought multiply land darkness had dry you're of."
            ]
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn grep_folder_exclude_dir() {
        let c = Config::new(
            "you",
            vec!["./testdata"],
            false,
            true,
            false,
            false,
            Some("folder"),
        );
        let mut r = grep(c).unwrap();
        r.sort();
        assert_eq!(
            r,
            vec![
                "./testdata/lol:Evening green fill you'll gathering above hath.",
                "./testdata/lol:He divide for appear deep abundantly. Had above unto. Moving stars fish. Whose you'll can't beginning sixth.",
                "./testdata/lol:Third dominion you're had called green.",
                "./testdata/lol:Tree brought multiply land darkness had dry you're of."
            ]
        );
    }
}
