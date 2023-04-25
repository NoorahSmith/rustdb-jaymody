mod db;
mod fstring;

use std::io::{self, Write};
use std::str::FromStr;

use crate::db::Row;

fn main() {
    loop {
        // prompt
        print!("\ndb > ");
        _ = io::stdout().flush();

        // get input str
        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer).unwrap();
        let input_str = input_buffer.as_str().trim();

        // process meta commands
        if input_str.starts_with(".") {
            match input_str {
                ".exit" => std::process::exit(0),
                _ => {
                    println!("Unrecognized command: {}", input_str);
                    continue;
                }
            };
        }

        // process statement
        if input_str.starts_with("insert") {
            match Row::from_str(input_str.strip_prefix("insert").unwrap()) {
                Ok(row) => println!("This is where we would do an insert. {}", row),
                Err(error) => println!("Failed to parse insert statement: {}", error),
            }
        } else if input_str.starts_with("select") {
            println!("This is where we would do a select.");
        } else {
            println!("Unrecognized keyword: {}", input_str);
        }
    }
}
