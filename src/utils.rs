use std::{env, io};

#[derive(Debug)]
pub struct GrepArgs {
    query: String,
    fp: String,
}

impl GrepArgs {
    pub fn new(query: String, fp: String) -> Self {
        Self { query, fp }
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

pub fn get_arguments() -> GrepArgs {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        GrepArgs::new(get_query().to_string(), get_filepath().to_string())
    } else {
        GrepArgs::new(args[1].clone(), args[2].clone())
    }
}
