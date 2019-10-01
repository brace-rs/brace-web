use std::fmt::Write;

use self::attribute::{Attr, Attrs};
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
            match val {
                Attr::String(string) => write!(renderer, " {}=\"{}\"", key, string)?,
                Attr::Boolean(boolean) => {
                    if *boolean {
                        write!(renderer, " {}", key)?;
                    }
                }
            }
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
    fn test_element_attribute_string() {
        let mut element_1 = Element::new("div");
        let mut element_2 = Element::with("div", (), ());

        element_1.attrs_mut().insert("class", "test_1");
        element_2.attrs_mut().insert("class", "test_2");

        assert_eq!(
            element_1.attrs().get("class").unwrap().as_string().unwrap(),
            "test_1"
        );
        assert_eq!(
            element_2.attrs().get("class").unwrap().as_string().unwrap(),
            "test_2"
        );
    }

    #[test]
    fn test_element_attribute_boolean() {
        let mut element_1 = Element::new("input");
        let mut element_2 = Element::with("input", (), ());

        element_1.attrs_mut().insert("selected", true);
        element_2.attrs_mut().insert("selected", false);

        assert_eq!(
            *element_1
                .attrs()
                .get("selected")
                .unwrap()
                .as_boolean()
                .unwrap(),
            true
        );
        assert_eq!(
            *element_2
                .attrs()
                .get("selected")
                .unwrap()
                .as_boolean()
                .unwrap(),
            false
        );
    }
}
