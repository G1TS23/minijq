mod parser;
mod value;

use crate::parser::Parser;
use crate::query::parser::QueryParser;
use std::io::IsTerminal;
use std::io::Read;

mod query;
#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();

    let query = match args.nth(1) {
        Some(query) => query,
        None if std::io::stdin().is_terminal() => {
            eprintln!("No query. Try: minijq <query> <file>");
            std::process::exit(1)
        }
        None => ".".to_string(),
    };

    let input = match args.next() {
        Some(path) => std::fs::read_to_string(path)?,
        None if std::io::stdin().is_terminal() => {
            eprintln!("No file or stdin. Try: minijq . <file>");
            std::process::exit(1)
        }
        None => read_stdin()?,
    };
    let mut parser = Parser::new(&input);
    let mut query_parser = QueryParser::new(&query);
    let value = parser.parse()?;
    let query = query_parser.parse()?;
    println!("{}", value);
    Ok(())
}

fn read_stdin() -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    if buffer.is_empty() {
        eprintln!(
            "Content empty. Try: minijq <query> <file> or via stdin: echo '...' | minijq <query>"
        );
        std::process::exit(1)
    } else {
        Ok(buffer)
    }
}
