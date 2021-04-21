use clap::{App, Arg};
use perg::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let matches = App::new("perg")
        .version("0.1.0")
        .author("Federico Guerinoni <guerinoni.federico@gmail.com>")
        .about("grep like tool. Search for PATTERNS in each FILE.")
        .arg(
            Arg::new("line-number")
                .long("line-number")
                .short('n')
                .about("print line number with output lines"),
        )
        .arg(
            Arg::new("PATTERNS")
                .about("can contain multiple patterns separated by newlines.")
                .required(true),
        )
        .arg(Arg::new("FILE").required(true).min_values(1))
        .get_matches();

    let c = Config::new(
        matches.value_of("PATTERNS").unwrap(),
        matches.values_of("FILE").unwrap().collect(),
        matches.is_present("line-number"),
    );

    match grep(c) {
        Ok(results) => {
            results.par_iter().for_each(|item| println!("{}", item));
        }
        Err(e) => println!("{}", e),
    }
}
