use markdown::{mdast::Node, to_mdast, ParseOptions};
use ratatui::{text::Text, widgets::Paragraph};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct MarkdownParagraph<'a> {
    paragraph: Paragraph<'a>,
}

impl<'a> MarkdownParagraph<'a> {
    fn node_to_text(node: Node) -> Text<'a> {
        todo!("Convert the markdown ast to a rartui text object")
    }

    pub fn new<T>(text: &'static str) -> Self {
        // let mut formated_text: Text = text.into();

        let Ok(markdown) = to_mdast(text, &ParseOptions::default()) else {
            return Self {
                paragraph: Paragraph::new(text),
            };
        };

        let formated_text = markdown;

        // What we want to do is first tokenize the text stream, and then convert it to formated raatui text

        Self {
            paragraph: Paragraph::new("markdown"),
        }
    }

    pub fn paragraph(&self) -> &Paragraph<'a> {
        &self.paragraph
    }
}
