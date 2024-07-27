pub mod config;

use config::Config;
use std::{error::Error, fs, process};

pub fn read_file(fp: &str) -> String {
    match fs::read_to_string(fp) {
        Ok(content) => return content,
        Err(e) => {
            eprintln!("Cannot read the file on {} - an error occured: {}", fp, e);
            process::exit(1);
        }
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.trim())
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let lower_q = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().as_str().contains(lower_q.as_str()))
        .map(|line| line.trim())
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = read_file(config.filepath());
    let result = if config.case_insensible() {
        search_case_insensitive(config.query(), &content)
    } else {
        search(config.query(), &content)
    };
    for line in result {
        println!("{line}")
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "cat";
        let content = "\
        Stray cat was in the garden
        Under the tree
        While the rain was pouring
        From the sky.
        Category.";
        assert_eq!(search(query, content), vec!["Stray cat was in the garden",]);
    }

    #[test]
    fn case_insensitive() {
        let query = "cat";
        let content = "\
        Stray cat was in the garden
        Under the tree
        While the rain was pouring
        From the sky. Category.";
        assert_eq!(
            search_case_insensitive(query, content),
            vec!["Stray cat was in the garden", "From the sky. Category."]
        );
    }
}
