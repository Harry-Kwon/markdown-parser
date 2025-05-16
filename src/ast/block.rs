pub enum BlockType {
    Document,
    Paragraph
}

pub trait Block<'a> {
    fn into_html(&self) -> String;
}
pub trait ContainerBlock<'a>: Block<'a> {
    fn add_child(&mut self, block: Box<dyn Block<'a> + 'a>);
}

pub struct Document<'a> {
    pub(crate) blocks: Vec<Box<dyn Block<'a> + 'a>>
}
impl<'a> Document<'a> {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }
}
impl<'a> Block<'a> for Document<'a> {
    fn into_html(&self) -> String {
        let mut html = String::new();
        for block in &self.blocks {
            html.push_str(block.into_html().as_str());
            html.push_str("\n");
        }
        return html;
    }
}
impl<'a> ContainerBlock<'a> for Document<'a> {
    fn add_child(&mut self, block: Box<dyn Block<'a> + 'a>) {
        self.blocks.push(block);
    }
}

pub struct Paragraph<'a> {
    pub(crate) text: &'a str
}
impl<'a> Paragraph<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}
impl<'a> Block<'a> for Paragraph<'a> {
    fn into_html(&self) -> String {
        return format!("<p>{}</p>", self.text);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document() {
        let root = Document{
            blocks: vec![]
        };
        let expected = String::from("");
        let actual = root.into_html();

        assert!(actual == expected, "actual=\"{}\" expected=\"{}\"", actual, expected);
    }

    #[test]
    fn test_paragraph() {
        let root = Document{
            blocks: vec![
                Box::new(
                    Paragraph{
                        text: "Hello world!"
                    }
                )
            ]
        };
        let expected = String::from("<p>Hello world!</p>\n");
        let actual = root.into_html();

        assert!(actual == expected, "actual=\"{}\" expected=\"{}\"", actual, expected);
    }
}
