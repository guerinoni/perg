use jwalk::WalkDir;
use std::collections::VecDeque;
use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, Read},
    path,
};
#[derive(Debug, Clone)]
pub struct Config<'a> {
    pattern: &'a str,
    filenames: Vec<&'a str>,
    line_number: bool,
    recursive: bool,
    recursive_following_symlink: bool,
    ignore_case: bool,
    exclude_dir: Option<&'a str>,
    after_context_num: Option<usize>,
    before_context_num: Option<usize>,
}

impl<'a> Config<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pattern: &'a str,
        filenames: Vec<&'a str>,
        line_number: bool,
        recursive: bool,
        recursive_following_symlink: bool,
        ignore_case: bool,
        exclude_dir: Option<&'a str>,
        after_context_num: Option<usize>,
        before_context_num: Option<usize>,
    ) -> Config<'a> {
        Config {
            pattern,
            filenames,
            line_number,
            recursive,
            recursive_following_symlink,
            ignore_case,
            exclude_dir,
            after_context_num,
            before_context_num,
        }
    }
}

#[derive(Debug, Clone)]
struct ContextItem {
    line_number: usize,
    line: String,
}
#[derive(Debug, Clone)]
struct FixedCapacityDeque<T> {
    data: VecDeque<T>,
    capacity: usize,
}

impl<T> FixedCapacityDeque<T> {
    fn new(capacity: usize) -> Self {
        FixedCapacityDeque {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn push(&mut self, item: T) {
        if self.data.len() == self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(item);
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

fn search_in_file(
    filename: &str,
    pattern: &'_ str,
    show_line_number: bool,
    show_filename: bool,
    ignore_case: bool,
    after_context_num: Option<usize>,
    before_context_num: Option<usize>,
) -> Vec<String> {
    println!("before_context_num: {:?}", &before_context_num);

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

    let mut pattern = pattern.to_string();
    if ignore_case {
        pattern = pattern.to_lowercase();
    }
    let mut pushed_map: HashMap<usize, bool> = HashMap::new();
    let show_context = after_context_num.is_some() || before_context_num.is_some();
    let mut brefore_contexts: Option<FixedCapacityDeque<ContextItem>> = None;
    if before_context_num.is_some() {
        brefore_contexts = Some(FixedCapacityDeque::new(before_context_num.unwrap() + 1));
    }

    let mut after_unpushed_contexts_map: HashMap<usize, bool> = HashMap::new();

    for (idx, str) in lines.enumerate() {
        if show_context && pushed_map.contains_key(&idx) {
            continue;
        }
        if let Ok(item) = str {
            let mut item_ = item.clone();

            if ignore_case {
                item_ = item_.to_lowercase();
            }

            if before_context_num.is_some() {
                brefore_contexts.as_mut().unwrap().push(ContextItem {
                    line_number: idx,
                    line: item_.clone(),
                });
            }

            if item_.contains(pattern.as_str()) {
                if before_context_num.is_some() {
                    brefore_contexts.clone().unwrap().iter().for_each(|item| {
                        if item.line_number == idx {
                            return;
                        }
                        if pushed_map.contains_key(&item.line_number) {
                            return;
                        }
                        let mut s = String::from("");
                        if show_filename {
                            s = format!("{}:", filename);
                        }
                        if show_line_number {
                            s = format!("{}{}: ", s, item.line_number + 1);
                        }
                        s.push_str(item.line.as_str());
                        items.push(s);
                        if show_context {
                            pushed_map.insert(item.line_number, true);
                        }
                    })
                }

                let mut s = String::from("");
                if show_filename {
                    s = format!("{}:", filename);
                }
                if show_line_number {
                    s = format!("{}{}: ", s, idx + 1);
                }

                s.push_str(item.as_str());
                items.push(s);
                if show_context {
                    pushed_map.insert(idx, true);
                }

                if after_context_num.is_some() {
                    for i in 1..=after_context_num.unwrap() {
                        after_unpushed_contexts_map.insert(idx + i, true);
                    }
                }
            } else if after_context_num.is_some() && after_unpushed_contexts_map.contains_key(&idx)
            {
                let mut s = String::from("");
                if show_filename {
                    s = format!("{}:", filename);
                }
                if show_line_number {
                    s = format!("{}{}: ", s, idx + 1);
                }

                s.push_str(item.as_str());
                items.push(s);
                if show_context {
                    pushed_map.insert(idx, true);
                }
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
            .parallelism(jwalk::Parallelism::RayonDefaultPool)
        {
            let entry = entry.unwrap();
            if !c.recursive_following_symlink && entry.path_is_symlink() {
                continue;
            }
            let path = entry.path();
            let p = path.to_str().unwrap();
            if let Some(exclude) = c.exclude_dir {
                if path
                    .parent()
                    .unwrap()
                    .ancestors()
                    .any(|p| p.ends_with(exclude))
                {
                    continue;
                }
            }

            let mut res = search_in_file(
                p,
                c.pattern,
                c.line_number,
                true,
                c.ignore_case,
                c.after_context_num,
                c.before_context_num,
            );
            items.append(&mut res);
        }
    }

    if c.pattern == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap_or_default();
    }

    if c.filenames.is_empty() || c.filenames.len() == 1 && c.filenames[0] == "-" {
        let mut pushed_map: HashMap<usize, bool> = HashMap::new();
        let show_context = c.after_context_num.is_some() || c.before_context_num.is_some();
        let mut brefore_contexts: Option<FixedCapacityDeque<ContextItem>> = None;
        if c.before_context_num.is_some() {
            brefore_contexts = Some(FixedCapacityDeque::new(c.before_context_num.unwrap() + 1));
        }

        let mut after_unpushed_contexts_map: HashMap<usize, bool> = HashMap::new();

        let stdin = io::stdin();
        for (idx, line) in stdin.lock().lines().enumerate() {
            if show_context && pushed_map.contains_key(&idx) {
                continue;
            }

            let item = line.unwrap_or_default();
            let mut item_ = item.clone();
            let mut pattern = c.pattern.to_string();
            if c.ignore_case {
                item_ = item_.to_lowercase();
                pattern = pattern.to_lowercase();
            }

            if c.before_context_num.is_some() {
                brefore_contexts.as_mut().unwrap().push(ContextItem {
                    line_number: idx,
                    line: item_.clone(),
                });
            }

            if item_.contains(pattern.as_str()) {
                if c.before_context_num.is_some() {
                    brefore_contexts.clone().unwrap().iter().for_each(|item| {
                        if item.line_number == idx {
                            return;
                        }
                        if pushed_map.contains_key(&item.line_number) {
                            return;
                        }
                        let mut s = String::from("");
                        if c.line_number {
                            s = format!("{}{}: ", s, item.line_number + 1);
                        }
                        s.push_str(item.line.as_str());
                        println!("{}", s);
                        if show_context {
                            pushed_map.insert(item.line_number, true);
                        }
                    })
                }

                let mut s = String::from("");
                if c.line_number {
                    s = format!("{}{}: ", s, idx + 1);
                }

                s.push_str(item.as_str());

                println!("{}", s);

                if show_context {
                    pushed_map.insert(idx, true);
                }

                if c.after_context_num.is_some() {
                    for i in 1..=c.after_context_num.unwrap() {
                        after_unpushed_contexts_map.insert(idx + i, true);
                    }
                }
            } else if c.after_context_num.is_some()
                && after_unpushed_contexts_map.contains_key(&idx)
            {
                let mut s = String::from("");
                if c.line_number {
                    s = format!("{}{}: ", s, idx + 1);
                }

                s.push_str(item.as_str());
                println!("{}", s);
                if show_context {
                    pushed_map.insert(idx, true);
                }
            }
        }
    }

    for filename in c.filenames {
        let path = path::Path::new(filename);
        if !path.exists() {
            println!("No such file or directory");
            return Err("No such file or directory");
        }

        let mut res = search_in_file(
            filename,
            c.pattern,
            c.line_number,
            false,
            c.ignore_case,
            c.after_context_num,
            c.before_context_num,
        );
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
    fn grep_single_file_with_after_context() {
        let c = Config::new(
            "federico",
            vec!["./Cargo.toml"],
            false,
            false,
            false,
            false,
            None,
            Some(1),
            None,
        );
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![
                String::from("authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"),
                String::from("edition = \"2024\"")
            ])
        )
    }
    #[test]
    fn grep_single_file_with_before_context() {
        let c = Config::new(
            "federico",
            vec!["./Cargo.toml"],
            false,
            false,
            false,
            false,
            None,
            None,
            Some(1),
        );
        let r = grep(c);
        assert_eq!(
            r,
            Ok(vec![
                String::from("version = \"0.5.0\""),
                String::from("authors = [\"Federico Guerinoni <guerinoni.federico@gmail.com>\"]"),
            ])
        )
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn grep_folder() {
        let c = Config::new("you", vec!["./testdata"], false, true, false, false, None,None,None);
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
        let c = Config::new("you", vec!["./testdata"], false, true, false, true, None,None,None);
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
        let c = Config::new("you", vec![], false, true, false, false, None,None,None);
        let r = grep(c);
        assert!(r.is_ok());
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn grep_folder_with_line_numbers() {
        let c = Config::new("you", vec!["./testdata"], true, true, false, false, None,None,None);
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
            None,
            None,
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
