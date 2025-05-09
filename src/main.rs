use std::io::{self};

use markdown_parser::parse_markdown;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let ast = parse_markdown(buffer.as_str());

    dbg!(ast);

    Ok(())
}
