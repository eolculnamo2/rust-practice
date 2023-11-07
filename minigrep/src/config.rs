use std::env;

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub query: &'a str,
    pub path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, &str> {
        let query = args.get(1).ok_or("Missing query")?;
        let path = args.get(2).ok_or("Missing path")?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            path,
            ignore_case,
        })
    }
}
