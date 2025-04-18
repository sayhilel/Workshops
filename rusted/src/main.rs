use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use colored::Colorize;
mod character;

fn read_character_info() -> (String, u8) {
    let mut name: String = String::new();
    let mut color_str: String = String::new();

    print!("Who are you talking to? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    print!("What is their color? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut color_str).unwrap();
    let color_str = color_str.trim();
    let color = u8::from_str_radix(color_str, 16).unwrap();
    (name, color)
}

fn read_dialog(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
     * Should cover:
     *  File I/O
     *  Function Calls
     *  Error Types
     *  Basic TUI / CLI
     *  Match Statements
     *  String Manipulation - Not added yet, might be too hard for them.
     *
     * Fun to cover:
     *  Serde
     *  Actual Error Checking.
     */

    // CLI arguments decide who to make!
    let args: Vec<String> = env::args().collect();

    let (name, color) = read_character_info();
    let dialog = read_dialog(&args[2]);
    let c = character::Character::new(name, color, dialog);

    c.converse(args[1].to_string().yellow());
    Ok(())
}
