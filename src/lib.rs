use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_extracts_config_from_args() -> Result<(), String> {
        let query = "search text";
        let filename = "filename";

        let args: Vec<String> = vec!["minigrep", query, filename]
            .iter()
            .map(|a| String::from(*a))
            .collect();

        assert_eq!(
            Config::new(&args)?,
            Config {
                query: String::from(query),
                filename: String::from(filename)
            }
        );
        Ok(())
    }

    #[test]
    fn config_returns_err_when_not_enough_args() {
        let args: Vec<String> = vec![];
        assert_eq!(Config::new(&args).err(), Some("not enough arguments"));
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
