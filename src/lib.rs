use std::io::Error;
use std::io::ErrorKind;
use std::fs;

const TEXT_ARGUMENT_INDEX: usize = 1;
const FILE_ARGUMENT_INDEX: usize = 2;

pub struct Config {
    pub file: String,
    pub text: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, Error> {
        if args.len() < 3 {
            return Err(Error::from(ErrorKind::InvalidInput));
        }

        let text = args[TEXT_ARGUMENT_INDEX].clone();
        let file = args[FILE_ARGUMENT_INDEX].clone();
        Ok(Config { file, text })
    }
}

pub fn run(config: &Config) -> Result<(), Error> {
    let contents = fs::read_to_string(&config.file)?;

    for line in search(&config.text, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_positive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    #[should_panic]
    fn search_negative() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["Rust:"], search(query, contents));
    }
}