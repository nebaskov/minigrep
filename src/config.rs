use std::io;

#[derive(Debug)]
pub struct Config {
    query: String,
    fp: String,
    cs: bool,
}

impl Config {
    pub fn new(args: &[String], case_insense: bool) -> Self {
        let config = if args.len() < 3 {
            let query = get_query();
            let fp = get_filepath();
            let cs = if case_insense {
                case_insense
            } else {
                get_case_sensitivity()
            };
            Self { query, fp, cs }
        } else {
            Self {
                query: args[1].clone(),
                fp: args[2].clone(),
                cs: case_insense,
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

    pub fn case_insensible(&self) -> bool {
        self.cs
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

fn get_case_sensitivity() -> bool {
    println!("Please specify if search should be case insensitive [y/n]?");
    loop {
        let mut case_insensitive = String::new();
        if let Err(_) = io::stdin().read_line(&mut case_insensitive) {
            println!("An error occured, please, try again.");
            continue;
        }
        break case_insensitive.trim().to_lowercase().contains("y");
    }
}
