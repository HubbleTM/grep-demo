use std::{env, process};
use grep_rust::{Config, run};
use std::io::Error;

fn main() {
    match Config::new(&env::args().collect()) {
        Ok(config) => if let Err(e) = run(&config) {
            println!("Application error: {}", e);
            process::exit(1)
        },

        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1)
        }
    }
}