use std::io::{self, Write};

use colored::{ColoredString, Colorize};

pub struct Character {
    name_styled: ColoredString,
    dialog: Vec<String>,
}

impl Character {
    pub fn new(name: String, color: u8, dialog: Vec<String>) -> Self {
        let name_styled = match color {
            1 => name.red(),
            2 => name.green(),
            3 => name.blue(),
            _ => name.normal(),
        };

        Self {
            name_styled,
            dialog,
        }
    }

    pub fn converse(&self, you: ColoredString) {
        let mut index = 0;
        let mut input = String::new();

        println!("[ {} and {} start talking. ]", you, self.name_styled);
        io::stdout().flush().unwrap();
        while index < self.dialog.len() {
            println!("{}: {}", self.name_styled, self.dialog[index]);
            io::stdout().flush().unwrap();

            print!("{}: ", you);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            index += 1;
            while self.dialog[index].len() == 0 && index < self.dialog.len() - 1 {
                index += 1;
            }
        }
    }
}
