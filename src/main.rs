use std::env;

use perg::*;

fn main() {
    if env::args().len() < 3 {
        println!("not enough args, required <string> <file>");
        return;
    }

    // let file = fs::File::open(filename).unwrap();

    let c = Config::new(env::args().nth(1).expect("").as_str(), env::args().nth(2).expect("").as_str());
    // let lines = io::BufReader::new(file).lines();

    // let want_search = env::args().nth(1).expect("");

    // let mut results = Vec::new();
    // for l in lines {
    //     if let Ok(line) = l {
    //         if line.contains(want_search.as_str()) {
    //             results.push(line);
    //         }
    //     }
    // }

    // for r in results {
    //     println!("{}", r);
    // }
}
