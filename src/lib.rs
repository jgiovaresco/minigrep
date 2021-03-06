mod config;

pub use config::Config;

use anyhow::{Context, Result};
use std::fs;

pub fn run(config: Config) -> Result<()> {
    let contents = fs::read_to_string(&config.path)
        .with_context(|| format!("could not read {}", config.path.display()))?;

    for line in search(&config.pattern, &contents) {
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
    fn search_returns_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
