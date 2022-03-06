use std::{
    env, fs,
    io::{self, BufRead},
    process::exit,
};

use recipe::Recipe;
use serde_json::Result;

mod md;
mod recipe;
mod scrape;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let input = {
        if filepath == "-" {
            read_stdin()
        } else {
            read_file(filepath)
        }
    };

    let recipe: Result<Recipe> = serde_json::from_str(&input);

    if let Ok(r) = recipe {
        println!("{}", r.to_md());
    } else {
        eprintln!("Unable to parse as a recipe.");
        exit(1);
    }
}

fn read_stdin() -> String {
    let mut buffer = String::new();

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            buffer.push_str(&line)
        }
    }

    buffer
}

fn read_file(filepath: &str) -> String {
    fs::read_to_string(filepath).expect(&format!("Could not read file: {}", filepath))
}
