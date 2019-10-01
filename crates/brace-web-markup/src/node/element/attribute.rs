use indexmap::map::{IndexMap, IntoIter, Iter, IterMut};

use crate::node::element::Element;
use crate::node::text::Text;
use crate::node::{Node, Nodes};

#[derive(Clone)]
pub enum Attr {
    String(String),
    Boolean(bool),
    Nodes(Nodes),
    None,
}

impl Attr {
    pub fn string<T>(string: T) -> Self
    where
        T: Into<String>,
    {
        Self::String(string.into())
    }

    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn boolean<T>(boolean: T) -> Self
    where
        T: Into<bool>,
    {
        Self::Boolean(boolean.into())
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Self::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    pub fn as_boolean_mut(&mut self) -> Option<&mut bool> {
        match self {
            Self::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    pub fn nodes<T>(nodes: T) -> Self
    where
        T: Into<Nodes>,
    {
        Self::Nodes(nodes.into())
    }

    pub fn is_nodes(&self) -> bool {
        match self {
            Self::Nodes(_) => true,
            _ => false,
        }
    }

    pub fn as_nodes(&self) -> Option<&Nodes> {
        match self {
            Self::Nodes(nodes) => Some(nodes),
            _ => None,
        }
    }

    pub fn as_nodes_mut(&mut self) -> Option<&mut Nodes> {
        match self {
            Self::Nodes(nodes) => Some(nodes),
            _ => None,
        }
    }

    pub const fn none() -> Self {
        Self::None
    }

    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}

impl From<()> for Attr {
    fn from(_: ()) -> Self {
        Self::None
    }
}

impl From<&str> for Attr {
    fn from(from: &str) -> Self {
        Self::String(from.to_owned())
    }
}

impl From<String> for Attr {
    fn from(from: String) -> Self {
        Self::String(from)
    }
}

impl From<bool> for Attr {
    fn from(from: bool) -> Self {
        Self::Boolean(from)
    }
}

impl From<Text> for Attr {
    fn from(from: Text) -> Self {
        Self::Nodes(from.into())
    }
}

impl From<Vec<Text>> for Attr {
    fn from(from: Vec<Text>) -> Self {
        Self::Nodes(from.into())
    }
}

impl From<Element> for Attr {
    fn from(from: Element) -> Self {
        Self::Nodes(from.into())
    }
}

impl From<Vec<Element>> for Attr {
    fn from(from: Vec<Element>) -> Self {
        Self::Nodes(from.into())
    }
}

impl From<Node> for Attr {
    fn from(from: Node) -> Self {
        Self::Nodes(from.into())
    }
}

impl From<Vec<Node>> for Attr {
    fn from(from: Vec<Node>) -> Self {
        Self::Nodes(from.into())
    }
}

impl From<Nodes> for Attr {
    fn from(from: Nodes) -> Self {
        Self::Nodes(from)
    }
}

#[derive(Clone, Default)]
pub struct Attrs(pub(crate) IndexMap<String, Attr>);

impl Attrs {
    pub fn new() -> Self {
        Self(IndexMap::new())
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

    pub fn insert<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<Attr>,
    {
        self.0.insert(key.into(), value.into());
        self
    }

    pub fn remove<K>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
        self.0.swap_remove(key.as_ref());
        self
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<'_, String, Attr> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, String, Attr> {
        self.0.iter_mut()
    }
}

impl IntoIterator for Attrs {
    type Item = (String, Attr);
    type IntoIter = IntoIter<String, Attr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Attrs {
    type Item = (&'a String, &'a Attr);
    type IntoIter = Iter<'a, String, Attr>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Attrs {
    type Item = (&'a String, &'a mut Attr);
    type IntoIter = IterMut<'a, String, Attr>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl From<()> for Attrs {
    fn from(_: ()) -> Self {
        Self::default()
    }
}
