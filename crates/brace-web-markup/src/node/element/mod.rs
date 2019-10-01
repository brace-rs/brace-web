use std::fmt::Write;

use self::attribute::Attrs;
use super::Nodes;
use crate::render::{Render, Renderer, Result as RenderResult};

pub mod attribute;

pub fn element<T, A, N>(tag: T, attrs: A, nodes: N) -> Element
where
    T: Into<String>,
    A: Into<Attrs>,
    N: Into<Nodes>,
{
    Element::with(tag, attrs, nodes)
}

#[derive(Clone)]
pub struct Element {
    tag: String,
    attrs: Attrs,
    nodes: Nodes,
}

impl Element {
    pub fn new<T>(tag: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            tag: tag.into(),
            attrs: Attrs::new(),
            nodes: Nodes::new(),
        }
    }

    pub fn with<T, A, N>(tag: T, attrs: A, nodes: N) -> Self
    where
        T: Into<String>,
        A: Into<Attrs>,
        N: Into<Nodes>,
    {
        Self {
            tag: tag.into(),
            attrs: attrs.into(),
            nodes: nodes.into(),
        }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn attrs(&self) -> &Attrs {
        &self.attrs
    }

    pub fn attrs_mut(&mut self) -> &mut Attrs {
        &mut self.attrs
    }

    pub fn nodes(&self) -> &Nodes {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut Nodes {
        &mut self.nodes
    }
}

impl Render for Element {
    fn render(&self, renderer: &mut Renderer) -> RenderResult {
        write!(renderer, "<{}", self.tag)?;

        for (key, val) in &self.attrs {
            write!(renderer, " {}=\"{}\"", key, val)?;
        }

        write!(renderer, ">")?;

        for node in &self.nodes {
            node.render(renderer)?;
        }

        write!(renderer, "</{}>", self.tag)?;

        Ok(())
    }
}

impl From<&str> for Element {
    fn from(from: &str) -> Self {
        Self {
            tag: from.to_owned(),
            attrs: Attrs::new(),
            nodes: Nodes::new(),
        }
    }
}

impl From<String> for Element {
    fn from(from: String) -> Self {
        Self {
            tag: from,
            attrs: Attrs::new(),
            nodes: Nodes::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn test_element_attributes() {
        let mut element_1 = Element::new("div");
        let mut element_2 = Element::with("div", (), ());

        element_1.attrs_mut().insert("class", "test_1");
        element_2.attrs_mut().insert("class", "test_2");

        assert_eq!(element_1.attrs().get("class").unwrap(), "test_1");
        assert_eq!(element_2.attrs().get("class").unwrap(), "test_2");
    }
}
