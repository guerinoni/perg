use clap::{App, Arg};
use perg::*;

const PATTERNS: &str = "PATTERNS";
const FILE: &str = "FILE";
const LINE_NUMBER: &str = "line-number";
const RECURSIVE: &str = "recursive";
const IGNORE_CASE: &str = "ignore-case";

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("perg")
        .version(VERSION)
        .author("Federico Guerinoni <guerinoni.federico@gmail.com>")
        .about("grep like tool. Search for PATTERNS in each FILE.")
        .arg(
            Arg::new(PATTERNS)
                .required(true)
                .about("can contain multiple patterns separated by newlines."),
        )
        .arg(
            Arg::new(FILE)
                .min_values(1)
                .about("when FILE is '-', read standard input."),
        )
        .arg(
            Arg::new(LINE_NUMBER)
                .long("line-number")
                .short('n')
                .about("print line number with output lines."),
        )
        .arg(
            Arg::new(RECURSIVE)
                .long("recursive")
                .short('r')
                .about("search recursive in folders."),
        )
        .arg(
            Arg::new(IGNORE_CASE)
                .long("ignore-case")
                .short('i')
                .about("ignore case distinctions in patterns and data."),
        )
        .get_matches();

    let c = Config::new(
        matches.value_of(PATTERNS).unwrap(),
        matches.values_of(FILE).unwrap_or_default().collect(),
        matches.is_present(LINE_NUMBER),
        matches.is_present(RECURSIVE),
        matches.is_present(IGNORE_CASE),
    );

    match grep(c) {
        Ok(results) => {
            results.iter().for_each(|item| println!("{}", item));
        }
        Err(e) => println!("{}", e),
    }
}
