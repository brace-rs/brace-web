pub enum Node {
    Text(Text),
    Element(Element),
}

pub struct Text {
    pub value: String,
}

impl Text {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self { value: text.into() }
    }
}

pub struct Element {
    pub tag: String,
    pub nodes: Vec<Node>,
}

impl Element {
    pub fn new<T>(tag: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            tag: tag.into(),
            nodes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Element, Node, Text};

    #[test]
    fn test_node_tree() {
        let mut html = Element::new("html");
        let mut body = Element::new("body");
        let text = Text::new("Hello world");

        body.nodes.push(Node::Text(text));

        assert_eq!(body.nodes.len(), 1);

        html.nodes.push(Node::Element(body));

        assert_eq!(html.nodes.len(), 1);
    }
}
