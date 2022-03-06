use std::io::{self, Read, BufRead};

use recipe::Recipe;
use serde_json::Result;

mod recipe;
mod md;
fn main() {
    let input = read_stdin();

    let recipe: Result<Recipe> = serde_json::from_str(&input);

    if let Ok(r) = recipe {
        println!("{}", r.to_md());
    } else {
        println!("{}", input);
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
