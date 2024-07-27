use minigrep::config::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let case_insense = env::var("CASE_INSENSITIVE").is_ok();
    dbg!(case_insense);
    let config = Config::new(&args, case_insense);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
