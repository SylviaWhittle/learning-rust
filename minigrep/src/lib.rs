use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // The 'static lifetime indicates that the variable can last
    // for the length of the program.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// The run function returns unit type (), or Box<dyn Error>
// Box<dyn Error> means that it will return a type that
// implements the Error trait, but we don't have to
// specify what particular type that will be.
// This provides flexibility to return errors of different
// types in different cases. The dyn keyword is short for dynamic.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Use the ? to return the error value on error.
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    // If success, return unit type ()
    Ok(())
}

// Note that here we clarify that the lifetime of the return strings will
// be the lifetime of the contents, and not the query.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        // search for the query in the line
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // Note that the \ at the start here tells rust not
        // to put a \n at the start of this string literal.
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
