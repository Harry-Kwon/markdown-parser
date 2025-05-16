use crate::ast::*;

fn parse_leaf(text: &str) -> Box<dyn Block + '_>{
    return Box::new(Paragraph::new(text));
}

pub fn parse_markdown<'a>(markdown: &'a str) -> Box<dyn Block<'a> + 'a> {
    let mut root = Box::new(Document::new());
    #[allow(unused_mut)] // mut needed when container blocks are implemented
    let mut current = &mut root;

    for line in markdown.lines() {
        let leaf = parse_leaf(line);
        current.add_child(leaf);
    }
    
    return root;
}
