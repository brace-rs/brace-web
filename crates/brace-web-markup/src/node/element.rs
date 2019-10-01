use std::collections::hash_map::{HashMap, IntoIter, Iter, IterMut};

use super::Nodes;

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

#[derive(Clone, Default)]
pub struct Attrs(HashMap<String, String>);

impl Attrs {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get<K>(&self, key: K) -> Option<&String>
    where
        K: AsRef<str>,
    {
        self.0.get(key.as_ref())
    }

    pub fn get_mut<K>(&mut self, key: K) -> Option<&mut String>
    where
        K: AsRef<str>,
    {
        self.0.get_mut(key.as_ref())
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.0.insert(key.into(), value.into());
        self
    }

    pub fn remove<K>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
        self.0.remove(key.as_ref());
        self
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<'_, String, String> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, String, String> {
        self.0.iter_mut()
    }
}

impl IntoIterator for Attrs {
    type Item = (String, String);
    type IntoIter = IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Attrs {
    type Item = (&'a String, &'a String);
    type IntoIter = Iter<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Attrs {
    type Item = (&'a String, &'a mut String);
    type IntoIter = IterMut<'a, String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl From<()> for Attrs {
    fn from(_: ()) -> Self {
        Self::default()
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
