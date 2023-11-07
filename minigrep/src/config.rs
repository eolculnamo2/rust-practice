use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, String> {
        let query = args.get(1).ok_or("Missing query".to_string())?;
        let path = args.get(2).ok_or("Missing path".to_string())?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query: query.clone(),
            path: path.clone(),
            ignore_case,
        })
    }
}
