use std::{
    env::{self, Args},
    error::Error,
    fs,
};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in if config.case_sensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    } {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(t) => t,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(t) => t,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .into_iter()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .into_iter()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "  
      Rust: 
      safe, fast, productive.
      Pick three.
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
