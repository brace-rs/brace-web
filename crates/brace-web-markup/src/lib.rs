pub enum Node {
    Text(Text),
    Element(Element),
}

impl Node {
    pub fn text<T>(text: T) -> Self
    where
        T: Into<Text>,
    {
        Self::Text(text.into())
    }

    pub fn is_text(&self) -> bool {
        match self {
            Self::Text(_) => true,
            _ => false,
        }
    }

    pub fn as_text(&self) -> Option<&Text> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn as_text_mut(&mut self) -> Option<&mut Text> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn element<T>(element: T) -> Self
    where
        T: Into<Element>,
    {
        Self::Element(element.into())
    }

    pub fn is_element(&self) -> bool {
        match self {
            Self::Element(_) => true,
            _ => false,
        }
    }

    pub fn as_element(&self) -> Option<&Element> {
        match self {
            Self::Element(element) => Some(element),
            _ => None,
        }
    }

    pub fn as_element_mut(&mut self) -> Option<&mut Element> {
        match self {
            Self::Element(element) => Some(element),
            _ => None,
        }
    }
}

impl From<Text> for Node {
    fn from(from: Text) -> Self {
        Self::Text(from)
    }
}

impl From<Element> for Node {
    fn from(from: Element) -> Self {
        Self::Element(from)
    }
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

impl From<&str> for Text {
    fn from(from: &str) -> Self {
        Self {
            value: from.to_owned(),
        }
    }
}

impl From<String> for Text {
    fn from(from: String) -> Self {
        Self { value: from }
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

impl From<&str> for Element {
    fn from(from: &str) -> Self {
        Self {
            tag: from.to_owned(),
            nodes: Vec::new(),
        }
    }
}

impl From<String> for Element {
    fn from(from: String) -> Self {
        Self {
            tag: from,
            nodes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Element, Node, Text};

    #[test]
    fn test_node_impl() {
        let mut node_1 = Node::Text(Text::new("hello world"));

        assert!(node_1.is_text());
        assert!(node_1.as_text().is_some());
        assert!(node_1.as_text_mut().is_some());

        let mut node_2 = Node::text("hello world");

        assert!(node_2.is_text());
        assert!(node_2.as_text().is_some());
        assert!(node_2.as_text_mut().is_some());

        let mut node_3 = Node::Element(Element::new("div"));

        assert!(node_3.is_element());
        assert!(node_3.as_element().is_some());
        assert!(node_3.as_element_mut().is_some());

        let mut node_4 = Node::element("div");

        assert!(node_4.is_element());
        assert!(node_4.as_element().is_some());
        assert!(node_4.as_element_mut().is_some());
    }

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

    #[test]
    fn test_node_tree_alternative() {
        let mut html = Node::element("html");
        let mut body = Node::element("body");
        let text = Node::text("hello world");

        body.as_element_mut().unwrap().nodes.push(text);

        assert_eq!(body.as_element_mut().unwrap().nodes.len(), 1);

        html.as_element_mut().unwrap().nodes.push(body);

        assert_eq!(html.as_element_mut().unwrap().nodes.len(), 1);
    }
}
