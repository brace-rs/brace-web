use super::Nodes;

pub fn element<T, U>(tag: T, nodes: U) -> Element
where
    T: Into<String>,
    U: Into<Nodes>,
{
    Element::with(tag, nodes)
}

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

    pub fn with<T, U>(tag: T, nodes: U) -> Self
    where
        T: Into<String>,
        U: Into<Nodes>,
    {
        Self {
            tag: tag.into(),
            nodes: nodes.into(),
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
