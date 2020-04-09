use std::collections::vec_deque::{IntoIter, Iter, IterMut, VecDeque};

use self::element::Element;
use self::text::Text;
use crate::render::{Render, Renderer, Result as RenderResult};

pub mod element;
pub mod text;

#[derive(Clone, Debug, PartialEq)]
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

impl Render for Node {
    fn render(&self, renderer: &mut Renderer) -> RenderResult {
        match self {
            Self::Text(text) => text.render(renderer),
            Self::Element(element) => element.render(renderer),
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Nodes(VecDeque<Node>);

impl Nodes {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn get(&self, index: usize) -> Option<&Node> {
        self.0.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.0.get_mut(index)
    }

    pub fn append<T>(&mut self, node: T) -> &mut Self
    where
        T: Into<Node>,
    {
        let node = node.into();

        if let Node::Text(text) = &node {
            if let Some(item) = self.0.back_mut().and_then(|item| item.as_text_mut()) {
                item.value_mut().push_str(text.value());

                return self;
            }
        }

        self.0.push_back(node);
        self
    }

    pub fn prepend<T>(&mut self, node: T) -> &mut Self
    where
        T: Into<Node>,
    {
        let mut node = node.into();

        if let Node::Text(text) = &mut node {
            if let Some(item) = self.0.front_mut().and_then(|item| item.as_text_mut()) {
                text.value_mut().push_str(item.value());

                *item.value_mut() = text.value().to_string();

                return self;
            }
        }

        self.0.push_front(node);
        self
    }

    pub fn remove(&mut self, index: usize) -> &mut Self {
        self.0.remove(index);
        self
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<'_, Node> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Node> {
        self.0.iter_mut()
    }
}

impl IntoIterator for Nodes {
    type Item = Node;
    type IntoIter = IntoIter<Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Nodes {
    type Item = &'a Node;
    type IntoIter = Iter<'a, Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Nodes {
    type Item = &'a mut Node;
    type IntoIter = IterMut<'a, Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl From<()> for Nodes {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<&str> for Nodes {
    fn from(from: &str) -> Self {
        Self(vec![Node::text(from)].into())
    }
}

impl From<String> for Nodes {
    fn from(from: String) -> Self {
        Self(vec![Node::text(from)].into())
    }
}

impl From<Text> for Nodes {
    fn from(from: Text) -> Self {
        Self(vec![Node::from(from)].into())
    }
}

impl From<Vec<Text>> for Nodes {
    fn from(from: Vec<Text>) -> Self {
        Self(from.into_iter().map(Node::from).collect())
    }
}

impl From<Element> for Nodes {
    fn from(from: Element) -> Self {
        Self(vec![Node::from(from)].into())
    }
}

impl From<Vec<Element>> for Nodes {
    fn from(from: Vec<Element>) -> Self {
        Self(from.into_iter().map(Node::from).collect())
    }
}

impl From<Node> for Nodes {
    fn from(from: Node) -> Self {
        Self(vec![from].into())
    }
}

impl From<&[Node]> for Nodes {
    fn from(from: &[Node]) -> Self {
        Self(from.to_vec().into())
    }
}

impl From<Vec<Node>> for Nodes {
    fn from(from: Vec<Node>) -> Self {
        Self(from.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Nodes;
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
    fn test_node_iter() {
        let mut nodes = Nodes::new();

        nodes.append(Node::element("div"));
        nodes.append(Node::element("span"));

        assert!(!nodes.is_empty());
        assert_eq!(nodes.len(), 2);

        let mut num = 0;

        for node in &nodes {
            assert!(node.is_element());

            num += 1;
        }

        for node in &mut nodes {
            assert!(node.is_element());

            num += 1;
        }

        for node in nodes {
            assert!(node.is_element());

            num += 1;
        }

        assert_eq!(num, 6);
    }

    #[test]
    fn test_node_tree() {
        let mut html = Element::new("html");
        let mut body = Element::new("body");
        let text = Text::new("Hello world");

        body.nodes_mut().append(Node::Text(text));

        assert_eq!(body.nodes().len(), 1);

        html.nodes_mut().append(Node::Element(body));

        assert_eq!(html.nodes().len(), 1);
    }

    #[test]
    fn test_node_tree_alternative() {
        let mut html = Node::element("html");
        let mut body = Node::element("body");
        let text = Node::text("hello world");

        body.as_element_mut().unwrap().nodes_mut().append(text);

        assert_eq!(body.as_element_mut().unwrap().nodes().len(), 1);

        html.as_element_mut().unwrap().nodes_mut().append(body);

        assert_eq!(html.as_element_mut().unwrap().nodes().len(), 1);
    }

    #[test]
    fn test_node_tree_nested() {
        let html: Node = Element::with(
            "html",
            (),
            Element::with("body", (), Text::new("hello world")),
        )
        .into();

        assert_eq!(html.as_element().unwrap().nodes().len(), 1);

        let body = html.as_element().unwrap().nodes().get(0).unwrap();

        assert_eq!(body.as_element().unwrap().nodes().len(), 1);

        let text = body.as_element().unwrap().nodes().get(0).unwrap();

        assert_eq!(text.as_text().unwrap().value(), "hello world");
    }

    #[test]
    fn test_insert_text() {
        let mut element_1 = Element::new("span");

        element_1.nodes_mut().append(Text::new("one"));
        element_1.nodes_mut().append(Text::new("two"));

        assert_eq!(element_1.nodes().len(), 1);
        assert_eq!(
            element_1.nodes().get(0).unwrap().as_text().unwrap().value(),
            "onetwo"
        );

        let mut element_2 = Element::new("span");

        element_2.nodes_mut().append(Text::new("one"));
        element_2.nodes_mut().prepend(Text::new("two"));

        assert_eq!(element_2.nodes().len(), 1);
        assert_eq!(
            element_2.nodes().get(0).unwrap().as_text().unwrap().value(),
            "twoone"
        );
    }
}
