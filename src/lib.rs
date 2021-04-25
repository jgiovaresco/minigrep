use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments"),
        };

        return Ok(Config { query, filename });
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query && self.filename == other.filename
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_extracts_config_from_args() -> Result<(), String> {
        let args = vec!["minigrep", "search text", "filename"];
        let args_iter = args.iter().map(|a| a.to_string());

        assert_eq!(
            Config::new(args_iter),
            Ok(Config {
                query: "search text".to_string(),
                filename: "filename".to_string()
            })
        );
        Ok(())
    }

    #[test]
    fn config_returns_err_when_not_enough_args() {
        let args = ["minigrep"].iter().map(|a| a.to_string());
        assert_eq!(Config::new(args), Err("not enough arguments"));
    }

    #[test]
    fn search_returns_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
