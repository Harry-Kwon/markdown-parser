use crate::blocks::*;

pub struct Parser;

impl Parser {
    pub fn parse_markdown(self, _text: &str) -> impl Block {
        return Document{};
    }
}
