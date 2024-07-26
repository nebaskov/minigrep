use std::{error::Error, fs, io, process};

#[derive(Debug)]
pub struct Config {
    query: String,
    fp: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let config = if args.len() < 3 {
            Self {
                query: get_query(),
                fp: get_filepath(),
            }
        } else {
            Self {
                query: args[1].clone(),
                fp: args[2].clone(),
            }
        };
        config
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn filepath(&self) -> &str {
        &self.fp
    }
}

fn get_query() -> String {
    println!("Please enter the query you want to search for:");
    loop {
        let mut query = String::new();
        if let Err(_) = io::stdin().read_line(&mut query) {
            println!("An error occured, please, try again.");
            continue;
        }
        break query.trim().to_string();
    }
}

fn get_filepath() -> String {
    println!("Please enter the filepath you want to search at:");
    loop {
        let mut fp = String::new();
        if let Err(_) = io::stdin().read_line(&mut fp) {
            println!("An error occured, please, try again.");
            continue;
        }
        break fp.trim().to_string();
    }
}

pub fn read_file(fp: &str) -> String {
    match fs::read_to_string(fp) {
        Ok(content) => return content,
        Err(e) => {
            println!("Cannot read the file on {} - an error occured: {}", fp, e);
            process::exit(1);
        }
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|x| x.contains(query)).collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = read_file(config.filepath());
    for l in search(config.query(), &content) {
        println!("{l}")
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        let query = "cat";
        let content = "\
        Stray cat was in the garden
        Under the tree
        While the rain was pouring
        From the sky.";
        assert_eq!(search(query, content), vec!["Stray cat was in the garden",]);
    }
}
