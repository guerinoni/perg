use clap::{App, Arg};
use perg::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let matches = App::new("perg")
        .version("0.1.0")
        .author("Federico Guerinoni <guerinoni.federico@gmail.com>")
        .about("grep like tool. Search for PATTERNS in each FILE.")
        .arg(
            Arg::new("PATTERNS")
                .required(true)
                .about("can contain multiple patterns separated by newlines."),
        )
        .arg(
            Arg::new("FILE")
                .min_values(1)
                .about("when FILE is '-', read standard input."),
        )
        .arg(
            Arg::new("line-number")
                .long("line-number")
                .short('n')
                .about("print line number with output lines."),
        )
        .arg(
            Arg::new("recursive")
                .long("recursive")
                .short('r')
                .about("search recursive in folders."),
        )
        .get_matches();

    let c = Config::new(
        matches.value_of("PATTERNS").unwrap(),
        matches.values_of("FILE").unwrap_or_default().collect(),
        matches.is_present("line-number"),
        matches.is_present("recursive"),
    );

    match grep(c) {
        Ok(results) => {
            results.par_iter().for_each(|item| println!("{}", item));
        }
        Err(e) => println!("{}", e),
    }
}
