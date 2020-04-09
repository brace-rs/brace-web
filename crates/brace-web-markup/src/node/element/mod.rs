use std::fmt::Write;
use std::mem::replace;
use std::ops::{Index, IndexMut};

use lazy_static::lazy_static;

use self::attribute::{Attr, Attrs};
use super::Nodes;
use crate::render::{Render, Renderer, Result as RenderResult};

pub mod attribute;

lazy_static! {
    static ref TAG: String = String::from("div");
    static ref NODES: Nodes = Nodes::new();
}

pub fn element<T, A, N>(tag: T, attrs: A, nodes: N) -> Element
where
    T: Into<String>,
    A: Into<Attrs>,
    N: Into<Nodes>,
{
    Element::with(tag, attrs, nodes)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element(Attrs);

impl Element {
    pub fn new<T>(tag: T) -> Self
    where
        T: Into<String>,
    {
        let mut attrs = Attrs::new();

        attrs.insert("tag".to_owned(), Attr::string(tag));
        attrs.insert("nodes".to_owned(), Attr::nodes(()));

        Self(attrs)
    }

    pub fn with<T, A, N>(tag: T, attrs: A, nodes: N) -> Self
    where
        T: Into<String>,
        A: Into<Attrs>,
        N: Into<Nodes>,
    {
        let mut attrs = attrs.into();

        attrs.insert("tag".to_owned(), Attr::string(tag));
        attrs.insert("nodes".to_owned(), Attr::nodes(nodes));

        Self(attrs)
    }

    pub fn tag(&self) -> &String {
        &self.0.get("tag").and_then(Attr::as_string).unwrap_or(&TAG)
    }

    pub fn attrs(&self) -> &Attrs {
        &self.0
    }

    pub fn attrs_mut(&mut self) -> &mut Attrs {
        &mut self.0
    }

    pub fn nodes(&self) -> &Nodes {
        self.0
            .get("nodes")
            .and_then(Attr::as_nodes)
            .unwrap_or(&NODES)
    }

    pub fn nodes_mut(&mut self) -> &mut Nodes {
        (self.0)
            .0
            .entry("nodes".to_owned())
            .and_modify(|item| {
                if !item.is_nodes() {
                    replace(item, Attr::nodes(()));
                }
            })
            .or_insert_with(|| Attr::nodes(()))
            .as_nodes_mut()
            .unwrap()
    }

    pub fn get<K>(&self, key: K) -> Option<&Attr>
    where
        K: AsRef<str>,
    {
        self.0.get(key.as_ref())
    }

    pub fn get_mut<K>(&mut self, key: K) -> Option<&mut Attr>
    where
        K: AsRef<str>,
    {
        self.0.get_mut(key.as_ref())
    }

    pub fn set<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<Attr>,
    {
        self.0.insert(key, val);
        self
    }

    pub fn attr<K, V>(mut self, key: K, val: V) -> Self
    where
        K: Into<String>,
        V: Into<Attr>,
    {
        self.0.insert(key, val);
        self
    }
}

impl Render for Element {
    fn render(&self, renderer: &mut Renderer) -> RenderResult {
        write!(renderer, "<{}", self.tag())?;

        for (key, val) in self.attrs() {
            if key != "tag" && key != "nodes" {
                match val {
                    Attr::String(string) => write!(renderer, " {}=\"{}\"", key, string)?,
                    Attr::Boolean(boolean) => {
                        if *boolean {
                            write!(renderer, " {}", key)?;
                        }
                    }
                    _ => (),
                }
            }
        }

        write!(renderer, ">")?;

        for node in self.nodes() {
            node.render(renderer)?;
        }

        write!(renderer, "</{}>", self.tag())?;

        Ok(())
    }
}

impl Index<&str> for Element {
    type Output = Attr;

    fn index(&self, index: &str) -> &Self::Output {
        static NONE: Attr = Attr::none();

        self.0.get(index).unwrap_or(&NONE)
    }
}

impl IndexMut<&str> for Element {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        (self.0)
            .0
            .entry(index.to_owned())
            .or_insert_with(Attr::none)
    }
}

impl Index<String> for Element {
    type Output = Attr;

    fn index(&self, index: String) -> &Self::Output {
        static NONE: Attr = Attr::none();

        self.0.get(&index).unwrap_or(&NONE)
    }
}

impl IndexMut<String> for Element {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        (self.0).0.entry(index).or_insert_with(Attr::none)
    }
}

impl From<&str> for Element {
    fn from(from: &str) -> Self {
        Self::new(from)
    }
}

impl From<String> for Element {
    fn from(from: String) -> Self {
        Self::new(from)
    }
}

impl From<(&str, Attrs, Nodes)> for Element {
    fn from(from: (&str, Attrs, Nodes)) -> Self {
        Self::with(from.0, from.1, from.2)
    }
}

#[cfg(test)]
mod tests {
    use super::attribute::Attr;
    use crate::{Element, Text};

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

    #[test]
    fn test_element_attribute_none() {
        let mut element_1 = Element::new("div");
        let mut element_2 = Element::with("div", (), ());

        element_1.attrs_mut().insert("attr", ());
        element_2.attrs_mut().insert("attr", ());

        assert!(element_1.attrs().get("attr").unwrap().is_none());
        assert!(element_2.attrs().get("attr").unwrap().is_none());
    }

    #[test]
    fn test_element_impl() {
        let element = Element::new("div")
            .attr("id", "test")
            .attr("class", "testing")
            .attr(
                "nodes",
                vec![
                    Element::new("span")
                        .attr("class", "one")
                        .attr("nodes", Text::new("one")),
                    Element::new("span")
                        .attr("class", "two")
                        .attr("nodes", Text::new("two")),
                ],
            );

        assert_eq!(element.nodes().len(), 2);
        assert_eq!(element.tag(), "div");
        assert_eq!(element.get("tag").unwrap().as_string().unwrap(), "div");
        assert_eq!(element.get("id").unwrap().as_string().unwrap(), "test");
        assert_eq!(
            element.get("class").unwrap().as_string().unwrap(),
            "testing"
        );

        let node_1 = element.nodes().get(0).unwrap().as_element().unwrap();

        assert_eq!(node_1.nodes().len(), 1);
        assert_eq!(node_1.tag(), "span");
        assert_eq!(node_1.get("tag").unwrap().as_string().unwrap(), "span");
        assert_eq!(node_1.get("class").unwrap().as_string().unwrap(), "one");

        let node_2 = element.nodes().get(1).unwrap().as_element().unwrap();

        assert_eq!(node_2.nodes().len(), 1);
        assert_eq!(node_2.tag(), "span");
        assert_eq!(node_2.get("tag").unwrap().as_string().unwrap(), "span");
        assert_eq!(node_2.get("class").unwrap().as_string().unwrap(), "two");
    }

    #[test]
    fn test_element_indexing() {
        let mut element = Element::new("div").attr("class", "testing");

        assert!(element["id"].is_none());
        assert!(element["class"].is_string());
        assert_eq!(element["class"].as_string().unwrap(), "testing");

        element["one"] = Attr::string("hello world");

        assert!(element["one"].is_string());
        assert_eq!(element["one"].as_string().unwrap(), "hello world");

        element["two"] = Text::new("hello world").into();

        assert!(element["two"].is_nodes());
        assert_eq!(element["two"].as_nodes().unwrap().len(), 1);

        element["three"] = vec![Element::new("div"), Element::new("span")].into();

        assert!(element["three"].is_nodes());
        assert_eq!(element["three"].as_nodes().unwrap().len(), 2);
    }
}
