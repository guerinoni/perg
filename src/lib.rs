pub struct Config<'a> {
    want_search: &'a str,
    filename: &'a str,
}

impl<'a> Config <'a> {
    pub fn new(want_search: &'a str, filename: &'a str) -> Config<'a> {
        Config {
            want_search,
            filename,
        }
    }
}
