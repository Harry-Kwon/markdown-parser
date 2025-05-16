use std::io::{self};

use markdown_parser::markdown_to_html;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let html = markdown_to_html(buffer.as_str());
    println!("{}", html);

    Ok(())
}
