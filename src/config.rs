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
}
