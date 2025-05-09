mod block;
mod container_blocks;
mod leaf_blocks;

pub use block::Block;
pub use container_blocks::*;
pub use leaf_blocks::*;

pub trait BlockVisitor {
    type Value;
    fn visit(&self, block: ThematicBreak) -> Self::Value;
}

struct HTMLVisitor;

impl BlockVisitor for HTMLVisitor {
    type Value = String;
    fn visit(&self, _block: ThematicBreak) -> Self::Value {
        return String::from("<hr />");
    }
}
