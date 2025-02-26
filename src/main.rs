use clap::{Arg, Command};
use perg::{Config, grep};
use clap::ArgAction::SetTrue;
const PATTERNS: &str = "PATTERNS";
const FILE: &str = "FILE";
const LINE_NUMBER: &str = "line-number";
const RECURSIVE: &str = "recursive";
const DEREFERENCE_RECURSIVE: &str = "dereference-recursive";
const IGNORE_CASE: &str = "ignore-case";
const EXCLUDE_DIR: &str = "exclude-dir";

const VERSION: &str = env!("CARGO_PKG_VERSION");

const AFTER_CONTEXT: &str = "after-context";
const BEFORE_CONTEXT: &str = "before-context";
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
            .num_args(1..)
                .help("when FILE is '-', read standard input"),
        )
        .arg(
            Arg::new(LINE_NUMBER)
                .long("line-number")
                .short('n')
                .help("print line number with output lines")
                .action(SetTrue)
                .display_order(1),
        )
        .arg(
            Arg::new(RECURSIVE)
                .long("recursive")
                .short('r')
                .help("search recursive in folders.")
                .action(SetTrue)
                .display_order(2),
        )
        .arg(
            Arg::new(DEREFERENCE_RECURSIVE)
                .long("dereference-recursive")
                .short('R')
                .help("likewise, but follow all symlinks")
                .action(SetTrue)
                .display_order(3),
        )
        .arg(
            Arg::new(EXCLUDE_DIR)
                .long("exclude-dir")
                .number_of_values(1)
                .help("skip directories that match GLOB")
                .display_order(4),
        )
        .arg(
            Arg::new(IGNORE_CASE)
                .long("ignore-case")
                .short('i')
                .help("ignore case distinctions in patterns and data.")
                .action(SetTrue)
                .display_order(0),
        )
        .arg(
            Arg::new(AFTER_CONTEXT)
                .long(AFTER_CONTEXT)
                .short('A')
                .help("print NUM lines of trailing context")
                .value_parser(clap::value_parser!(usize))
                .display_order(5),
        )
        .arg(
            Arg::new(BEFORE_CONTEXT)
                .long(BEFORE_CONTEXT)
                .short('B')
                .help("print NUM lines of leading context")
                .value_parser(clap::value_parser!(usize))
                .display_order(6),
        )
        .get_matches();

        let patterns = matches.get_one::<String>(PATTERNS).expect("patterns are required");
        let files: Vec<String> = matches.get_many::<String>(FILE)
            .map(|vals| vals.cloned().collect())
            .unwrap_or_default();
        let file_refs: Vec<&str> = files.iter().map(String::as_str).collect();

    let c = Config::new(
        patterns,
        file_refs,
        matches.get_flag(LINE_NUMBER),
        matches.get_flag(RECURSIVE),
        matches.get_flag(DEREFERENCE_RECURSIVE),
        matches.get_flag(IGNORE_CASE),
        matches.get_one::<&str>(EXCLUDE_DIR).cloned(),
        matches.get_one::<usize>(AFTER_CONTEXT).copied(),
        matches.get_one::<usize>(BEFORE_CONTEXT).copied(),
    );


    match grep(c) {
        Ok(results) => {
            results.iter().for_each(|item| println!("{}", item));
        }
        Err(e) => println!("{}", e),
    }
}
