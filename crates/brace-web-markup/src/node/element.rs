use super::Nodes;

#[derive(Clone)]
pub struct Element {
    pub tag: String,
    nodes: Nodes,
}

impl Element {
    pub fn new<T>(tag: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            tag: tag.into(),
            nodes: Nodes::new(),
        }
    }

    pub fn nodes(&self) -> &Nodes {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut Nodes {
        &mut self.nodes
    }
}

impl From<&str> for Element {
    fn from(from: &str) -> Self {
        Self {
            tag: from.to_owned(),
            nodes: Nodes::new(),
        }
    }
}

impl From<String> for Element {
    fn from(from: String) -> Self {
        Self {
            tag: from,
            nodes: Nodes::new(),
        }
    }
}
