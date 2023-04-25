use std::path::Path;
use std::str::FromStr;

use crate::db::Row;
use crate::db::Table;

pub fn start<R, W>(mut reader: R, mut writer: W)
where
    R: std::io::BufRead,
    W: std::io::Write,
{
    let mut table: Table = Table::open(Path::new("temp.db"));

    loop {
        // prompt
        write!(writer, "\n{}rustdb >{} ", "\x1b[01;32m", "\x1b[00m").unwrap();
        writer.flush().unwrap();

        // get input str
        let mut input_buffer = String::new();
        reader.read_line(&mut input_buffer).unwrap();
        let input_str = input_buffer.as_str().trim();

        // process meta commands
        if input_str.starts_with(".") {
            match input_str {
                ".exit" => {
                    if let Err(msg) = table.close() {
                        writeln!(writer, "{}", msg).unwrap();
                    }
                    std::process::exit(0)
                }
                _ => {
                    writeln!(writer, "Unrecognized command: {}", input_str).unwrap();
                    continue;
                }
            };
        }

        // process statement
        if input_str.starts_with("insert") {
            match Row::from_str(input_str.strip_prefix("insert").unwrap()) {
                Ok(row) => {
                    table.insert_row(row).unwrap();
                    writeln!(writer, "Executed.").unwrap();
                }
                Err(error) => {
                    writeln!(writer, "Failed to parse insert statement: {}", error).unwrap();
                }
            }
        } else if input_str.starts_with("select") {
            writeln!(writer, "{}", table.select()).unwrap();
            writeln!(writer, "Executed.").unwrap();
        } else {
            writeln!(writer, "Unrecognized keyword: {}", input_str).unwrap();
        }
    }
}
