use super::block::Block;

/// Special block for the root of the AST
#[derive(Debug)]
pub struct Document;
impl Block for Document{}
