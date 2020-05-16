use std::fmt::Write;
use std::ops::{Index, IndexMut};

use crate::node::attribute::{Attribute, Attributes};
use crate::node::{Node, Nodes};
use crate::render::{Render, Renderer, Result as RenderResult};

pub fn element<T>(tag: T) -> Element
where
    T: Into<String>,
{
    Element::new(tag)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    tag: String,
    attrs: Attributes,
    nodes: Nodes,
}

impl Element {
    pub fn new<T>(tag: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            tag: tag.into(),
            attrs: Attributes::new(),
            nodes: Nodes::new(),
        }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn attrs(&self) -> &Attributes {
        &self.attrs
    }

    pub fn attrs_mut(&mut self) -> &mut Attributes {
        &mut self.attrs
    }

    pub fn get_attr<K>(&self, key: K) -> Option<&Attribute>
    where
        K: AsRef<str>,
    {
        self.attrs.get(key)
    }

    pub fn get_attr_mut<K>(&mut self, key: K) -> Option<&mut Attribute>
    where
        K: AsRef<str>,
    {
        self.attrs.get_mut(key)
    }

    pub fn set_attr<K, V>(&mut self, key: K, attr: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<Attribute>,
    {
        self.attrs.set(key, attr);
        self
    }

    pub fn unset_attr<K, V>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
        self.attrs.unset(key);
        self
    }

    pub fn with_attr<K, V>(mut self, key: K, attr: V) -> Self
    where
        K: Into<String>,
        V: Into<Attribute>,
    {
        self.attrs.set(key, attr);
        self
    }

    pub fn with_attrs<T>(mut self, attrs: T) -> Self
    where
        T: IntoIterator<Item = (String, Attribute)>,
    {
        self.attrs.extend(attrs);
        self
    }

    pub fn nodes(&self) -> &Nodes {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut Nodes {
        &mut self.nodes
    }

    pub fn get_node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    pub fn get_node_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }

    pub fn with_node<T>(mut self, node: T) -> Self
    where
        T: Into<Node>,
    {
        self.nodes.append(node.into());
        self
    }

    pub fn with_nodes<T>(mut self, nodes: T) -> Self
    where
        T: IntoIterator<Item = Node>,
    {
        self.nodes.extend(nodes);
        self
    }
}

impl Render for Element {
    fn render(&self, renderer: &mut Renderer) -> RenderResult {
        write!(renderer, "<{}", self.tag())?;

        for (key, val) in self.attrs() {
            match val {
                Attribute::String(string) => write!(renderer, " {}=\"{}\"", key, string)?,
                Attribute::Boolean(boolean) => {
                    if *boolean {
                        write!(renderer, " {}", key)?;
                    }
                }
                _ => (),
            }
        }

        match self.tag() {
            "area" | "base" | "br" | "col" | "command" | "embed" | "hr" | "img" | "input"
            | "keygen" | "link" | "meta" | "param" | "source" | "track" | "wbr" => {
                write!(renderer, " />")?;
            }
            _ => {
                write!(renderer, ">")?;

                for node in self.nodes() {
                    node.render(renderer)?;
                }

                write!(renderer, "</{}>", self.tag())?;
            }
        }

        Ok(())
    }
}

impl Index<&str> for Element {
    type Output = Attribute;

    fn index(&self, index: &str) -> &Self::Output {
        static NONE: Attribute = Attribute::none();

        self.attrs.get(index).unwrap_or(&NONE)
    }
}

impl IndexMut<&str> for Element {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.attrs
            .entry(index.to_owned())
            .or_insert_with(Attribute::none)
    }
}

impl Index<String> for Element {
    type Output = Attribute;

    fn index(&self, index: String) -> &Self::Output {
        static NONE: Attribute = Attribute::none();

        self.attrs.get(&index).unwrap_or(&NONE)
    }
}

impl IndexMut<String> for Element {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        self.attrs.entry(index).or_insert_with(Attribute::none)
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

impl From<(&str, Attributes, Nodes)> for Element {
    fn from(from: (&str, Attributes, Nodes)) -> Self {
        Self::new(from.0).with_attrs(from.1).with_nodes(from.2)
    }
}

macro_rules! elements {
    ( $($name:ident)* ) => {
        $(
            #[cfg_attr(tarpaulin, skip)]
            pub fn $name() -> Element {
                Element::new(stringify!($name))
            }
        )*
    };
}

elements! {
    a abbr address area article aside audio b base bdi bdo blockquote body br button canvas caption
    cite code col colgroup data datalist dd del details dfn dialog div dl dt em embed fieldset
    figcaption figure footer form h1 h2 h3 h4 h5 h6 head header hgroup hr i iframe img input ins
    kbd label legend li link main map mark menu menuitem meta meter nav noscript object ol optgroup
    option output p param picture pre progress q rb rp rt rtc ruby s samp script section select
    slot small source span strong style sub summary sup table tbody td template textarea tfoot th
    thead time title tr track u ul var video wbr
}

elements! {
    path circle ellipse line polygon polyline rect image
}

#[cfg_attr(tarpaulin, skip)]
pub fn svg() -> Element {
    Element::new("svg").with_attr("xmlns", "http://www.w3.org/2000/svg")
}

#[cfg(test)]
mod tests {
    use crate::node::attribute::Attribute;
    use crate::node::element::Element;
    use crate::node::text::Text;

    #[test]
    fn test_element_attribute_string() {
        let mut element = Element::new("div");

        element.attrs_mut().set("class", "test_1");

        assert_eq!(
            element.attrs().get("class").unwrap().as_string().unwrap(),
            "test_1"
        );
    }

    #[test]
    fn test_element_attribute_boolean() {
        let mut element_1 = Element::new("input");
        let mut element_2 = Element::new("input");

        element_1.attrs_mut().set("selected", true);
        element_2.attrs_mut().set("selected", false);

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
        let mut element = Element::new("div");

        element.attrs_mut().set("attr", ());

        assert!(element.attrs().get("attr").unwrap().is_none());
    }

    #[test]
    fn test_element_impl() {
        let element = Element::new("div")
            .with_attr("id", "test")
            .with_attr("class", "testing")
            .with_nodes(vec![
                Element::new("span")
                    .with_attr("class", "one")
                    .with_node(Text::new("one"))
                    .into(),
                Element::new("span")
                    .with_attr("class", "two")
                    .with_node(Text::new("two"))
                    .into(),
            ]);

        assert_eq!(element.nodes().len(), 2);
        assert_eq!(element.tag(), "div");
        assert_eq!(element.get_attr("id").unwrap().as_string().unwrap(), "test");
        assert_eq!(
            element.get_attr("class").unwrap().as_string().unwrap(),
            "testing"
        );

        let node_1 = element.nodes().get(0).unwrap().as_element().unwrap();

        assert_eq!(node_1.nodes().len(), 1);
        assert_eq!(node_1.tag(), "span");
        assert_eq!(
            node_1.get_attr("class").unwrap().as_string().unwrap(),
            "one"
        );

        let node_2 = element.nodes().get(1).unwrap().as_element().unwrap();

        assert_eq!(node_2.nodes().len(), 1);
        assert_eq!(node_2.tag(), "span");
        assert_eq!(
            node_2.get_attr("class").unwrap().as_string().unwrap(),
            "two"
        );
    }

    #[test]
    fn test_element_indexing() {
        let mut element = Element::new("div").with_attr("class", "testing");

        assert!(element["id"].is_none());
        assert!(element["class"].is_string());
        assert_eq!(element["class"].as_string().unwrap(), "testing");

        element["one"] = Attribute::string("hello world");

        assert!(element["one"].is_string());
        assert_eq!(element["one"].as_string().unwrap(), "hello world");

        element["two"] = "hello universe".into();

        assert!(element["two"].is_string());
        assert_eq!(element["two"].as_string().unwrap(), "hello universe");
    }
}
