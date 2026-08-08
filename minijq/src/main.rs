mod parser;
mod value;

use crate::parser::Parser;
use std::io::IsTerminal;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();

    let input = match args.nth(1) {
        Some(path) => std::fs::read_to_string(path)?,
        None if std::io::stdin().is_terminal() => {
            eprintln!("No file or stdin. Try: mini-jq <file>");
            std::process::exit(1)
        }
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            if buffer.is_empty() {
                eprintln!("Content empty. Try: mini-jq <file> or via stdin: echo '...' | mini-jq");
                std::process::exit(1)
            } else {
                buffer
            }
        }
    };
    let mut parser = Parser::new(&input);
    let value = parser.parse()?;
    println!("parsed value :{:#?}", value);
    Ok(())
}
