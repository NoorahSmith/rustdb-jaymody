mod db;
mod fstring;
mod repl;

fn main() {
    let input = std::io::stdin().lock();
    let output = std::io::stdout();
    repl::start(input, output);
}
