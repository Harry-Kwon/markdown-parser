pub mod parser;
mod blocks;

use blocks::Block;

pub fn parse_markdown(text: &str) -> impl Block {
    let md_parser = parser::Parser{};
    let ast = md_parser.parse_markdown(text);
    return ast;
}
