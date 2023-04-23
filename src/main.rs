use std::io::{self, Write};

fn main() {
    loop {
        // prompt
        print!("\ndb > ");
        _ = io::stdout().flush();

        // get input str
        let mut input_buffer = String::new();
        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line");
        let input_str = input_buffer.as_str().trim();

        // response
        match input_str {
            ".exit" => break,
            _ => println!("Unrecognized command: {}", input_str),
        }
    }
}
