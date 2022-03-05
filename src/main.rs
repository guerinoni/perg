use clap::{Arg, Command};
use perg::{grep, Config};

const PATTERNS: &str = "PATTERNS";
const FILE: &str = "FILE";
const LINE_NUMBER: &str = "line-number";
const RECURSIVE: &str = "recursive";
const DEREFERENCE_RECURSIVE: &str = "dereference-recursive";
const IGNORE_CASE: &str = "ignore-case";
const EXCLUDE_DIR: &str = "exclude-dir";

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = Command::new("perg")
        .version(VERSION)
        .author("Federico Guerinoni <guerinoni.federico@gmail.com>")
        .about("grep like tool. Search for PATTERNS in each FILE.")
        .arg(
            Arg::new(PATTERNS)
                .required(true)
                .help("can contain multiple patterns separated by newlines"),
        )
        .arg(
            Arg::new(FILE)
                .min_values(1)
                .help("when FILE is '-', read standard input"),
        )
        .arg(
            Arg::new(LINE_NUMBER)
                .long("line-number")
                .short('n')
                .help("print line number with output lines")
                .display_order(1),
        )
        .arg(
            Arg::new(RECURSIVE)
                .long("recursive")
                .short('r')
                .help("search recursive in folders.")
                .display_order(2),
        )
        .arg(
            Arg::new(DEREFERENCE_RECURSIVE)
                .long("dereference-recursive")
                .short('R')
                .help("likewise, but follow all symlinks")
                .display_order(3),
        )
        .arg(
            Arg::new(EXCLUDE_DIR)
                .long("--exclude-dir")
                .number_of_values(1)
                .help("skip directories that match GLOB")
                .display_order(4),
        )
        .arg(
            Arg::new(IGNORE_CASE)
                .long("ignore-case")
                .short('i')
                .help("ignore case distinctions in patterns and data.")
                .display_order(0),
        )
        .get_matches();

    let c = Config::new(
        matches.value_of(PATTERNS).unwrap(),
        matches.values_of(FILE).unwrap_or_default().collect(),
        matches.is_present(LINE_NUMBER),
        matches.is_present(RECURSIVE),
        matches.is_present(DEREFERENCE_RECURSIVE),
        matches.is_present(IGNORE_CASE),
        matches.value_of(EXCLUDE_DIR),
    );

    match grep(c) {
        Ok(results) => {
            results.iter().for_each(|item| println!("{}", item));
        }
        Err(e) => println!("{}", e),
    }
}
