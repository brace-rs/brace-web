use std::collections::VecDeque;

use self::element::Element;
use self::text::Text;

pub mod element;
pub mod text;

#[derive(Clone)]
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

#[derive(Clone, Default)]
pub struct Nodes {
    inner: VecDeque<Node>,
}

impl Nodes {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&Node> {
        self.inner.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.inner.get_mut(index)
    }

    pub fn append<T>(&mut self, node: T) -> &mut Self
    where
        T: Into<Node>,
    {
        self.inner.push_back(node.into());
        self
    }

    pub fn prepend<T>(&mut self, node: T) -> &mut Self
    where
        T: Into<Node>,
    {
        self.inner.push_front(node.into());
        self
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<()> for Nodes {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<&str> for Nodes {
    fn from(from: &str) -> Self {
        Self {
            inner: vec![Node::text(from)].into(),
        }
    }
}

impl From<String> for Nodes {
    fn from(from: String) -> Self {
        Self {
            inner: vec![Node::text(from)].into(),
        }
    }
}

impl From<Text> for Nodes {
    fn from(from: Text) -> Self {
        Self {
            inner: vec![Node::from(from)].into(),
        }
    }
}

impl From<Element> for Nodes {
    fn from(from: Element) -> Self {
        Self {
            inner: vec![Node::from(from)].into(),
        }
    }
}

impl From<Node> for Nodes {
    fn from(from: Node) -> Self {
        Self {
            inner: vec![from].into(),
        }
    }
}

impl From<&[Node]> for Nodes {
    fn from(from: &[Node]) -> Self {
        Self {
            inner: from.to_vec().into(),
        }
    }
}

impl From<Vec<Node>> for Nodes {
    fn from(from: Vec<Node>) -> Self {
        Self { inner: from.into() }
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
}
