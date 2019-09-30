use crate::node::Node;

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
