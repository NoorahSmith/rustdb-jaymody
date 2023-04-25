use std::path::Path;
use std::str::FromStr;

use crate::db::row::Row;
use crate::db::table::Table;

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
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let input = input.as_str().trim();

        // process meta commands
        if input.starts_with(".") {
            match input {
                ".exit" => {
                    if let Err(msg) = table.close() {
                        writeln!(writer, "{}", msg).unwrap();
                    }
                    std::process::exit(0)
                }
                _ => {
                    writeln!(writer, "Unrecognized command: {}", input).unwrap();
                    continue;
                }
            };
        }

        // process statement
        if input.starts_with("insert") {
            match Row::from_str(input.strip_prefix("insert").unwrap()) {
                Ok(row) => {
                    table.insert_row(row).unwrap();
                    writeln!(writer, "Executed.").unwrap();
                }
                Err(error) => {
                    writeln!(writer, "Failed to parse insert statement: {}", error).unwrap();
                }
            }
        } else if input.starts_with("select") {
            writeln!(writer, "{}", table.select()).unwrap();
            writeln!(writer, "Executed.").unwrap();
        } else {
            writeln!(writer, "Unrecognized keyword: {}", input).unwrap();
        }
    }
}
