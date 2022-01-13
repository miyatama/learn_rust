use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Config {
    query: String, 
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(
            Config {
                query,   
                filename,
                case_sensitive,
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("searching: {}\nin {}", config.query, config.filename);

    println!("In file: {}", config.filename);
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let search_result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };
    println!("With text: {:?}", search_result);
    Ok(())
}

pub fn reverse_write(config: Config) -> Result<(), Box<dyn Error>> {
    let newfilename = format!("{}_new", config.filename);

    let mut file = File::open(config.filename).expect("file not found");
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)
        .expect("something went wrong reading the file");

    let mut write_file = File::create(newfilename)?;
    contents.reverse();
    write_file.write_all(&contents)
        .expect("can not write");
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast and productive
Pick three";
        assert_eq!(
            vec!["safe, fast and productive"], 
            search(query, contents)
        );
    }

    #[test]
    fn test_one_result_case_insensitive() {
        let query = "duct";
        let contents = "\
RUST:
SAFE, FAST AND PRODUCTIVE
PICK THREE";
        assert_eq!(
            vec!["SAFE, FAST AND PRODUCTIVE"], 
            search_insensitive(query, contents)
        );
    }
}
