use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();

    if let Some(path) = args.nth(1) {
        let file_content = std::fs::read_to_string(path)?;
        println!("{}", file_content);
        Ok(())
    } else {
        eprintln!("Missing argument:");
        eprintln!("Usage: mini-jq <file>");
        std::process::exit(1)
    }
}
