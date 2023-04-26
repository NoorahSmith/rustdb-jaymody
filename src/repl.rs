use std::path::Path;
use std::str::FromStr;

use crate::db::row::Row;
use crate::db::table::Table;

const PROMPT: &str = "\n\x1b[01;32mrustdb >\x1b[00m ";

pub fn start<R, W>(mut reader: R, mut writer: W)
where
    R: std::io::BufRead,
    W: std::io::Write,
{
    let mut table: Table = Table::open(Path::new("temp.db"));

    loop {
        // prompt
        write!(writer, "{}", PROMPT).unwrap();
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
                    return;
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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn run_lines(input_lines: &Vec<String>) -> Vec<String> {
        let input = input_lines.join("\n");
        let input = input.as_bytes();

        let mut output = Vec::new();
        start(&input[..], &mut output);

        let output_str = String::from_utf8(output).unwrap();
        let mut output_lines: Vec<&str> = output_str.split(&format!("{}", PROMPT)).collect();

        // get rid of anything that gets printed before first prompt is printed
        // then check that there is exactly 1 output line per input line
        output_lines.remove(0);
        assert_eq!(input_lines.len(), output_lines.len());

        // remove any trailing whitespace and convert to owned String
        let output_lines: Vec<String> = output_lines
            .iter()
            .map(|s| s.trim_end().to_string())
            .collect();

        output_lines
    }

    fn check_input_output_pairs(pairs: Vec<(&str, &str)>) {
        let (input_lines, expected_output): (Vec<String>, Vec<String>) = pairs
            .into_iter()
            .map(|x| (x.0.to_owned(), x.1.to_owned()))
            .unzip();

        let actual_output = run_lines(&input_lines);

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn insert_and_retrieve_row() {
        check_input_output_pairs(vec![
            ("insert 1 user1 person1@example.com", "Executed."),
            ("select", "(1, user1, person1@example.com)\n\nExecuted."),
            (".exit", ""),
        ]);
    }
}
