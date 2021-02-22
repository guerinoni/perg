use std::env;

use perg::*;

fn main() {
    if env::args().len() < 3 {
        println!("not enough args, required <string> <file>");
        return;
    }

    let want_search = env::args().nth(1).expect("");
    let filename = env::args().nth(2).expect("");

    let c = Config::new(want_search.as_str(), filename.as_str());

    let res = grep(c);
    for r in res {
        println!("{}", r);
    }
}
