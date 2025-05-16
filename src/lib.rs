pub mod parser;
mod ast;
use ast::*;

pub fn parse_markdown(text: &str) -> Box<dyn Block + '_>{
    let ast = parser::parse_markdown(text);
    return ast;
}

pub fn ast_to_html<'a>(root: &impl Block<'a>) -> String {
    return root.into_html()
}

pub fn markdown_to_html(text: &str) -> String {
    let ast = parser::parse_markdown(text);
    return ast.into_html();
}
