use indexmap::map::{Entry, IndexMap, IntoIter, Iter, IterMut};

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    String(String),
    Boolean(bool),
    None,
}

impl Attribute {
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

impl From<()> for Attribute {
    fn from(_: ()) -> Self {
        Self::None
    }
}

impl From<&str> for Attribute {
    fn from(from: &str) -> Self {
        Self::string(from)
    }
}

impl From<String> for Attribute {
    fn from(from: String) -> Self {
        Self::string(from)
    }
}

impl From<bool> for Attribute {
    fn from(from: bool) -> Self {
        Self::boolean(from)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attributes(IndexMap<String, Attribute>);

impl Attributes {
    pub fn new() -> Self {
        Self(IndexMap::new())
    }

    pub fn get<K>(&self, key: K) -> Option<&Attribute>
    where
        K: AsRef<str>,
    {
        self.0.get(key.as_ref())
    }

    pub fn get_mut<K>(&mut self, key: K) -> Option<&mut Attribute>
    where
        K: AsRef<str>,
    {
        self.0.get_mut(key.as_ref())
    }

    pub fn set<K, V>(&mut self, key: K, attr: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<Attribute>,
    {
        self.0.insert(key.into(), attr.into());
        self
    }

    pub fn unset<K>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
        self.0.shift_remove(key.as_ref());
        self
    }

    pub fn entry<K>(&mut self, key: K) -> Entry<String, Attribute>
    where
        K: Into<String>,
    {
        self.0.entry(key.into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<'_, String, Attribute> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, String, Attribute> {
        self.0.iter_mut()
    }
}

impl Extend<(String, Attribute)> for Attributes {
    fn extend<I: IntoIterator<Item = (String, Attribute)>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl IntoIterator for Attributes {
    type Item = (String, Attribute);
    type IntoIter = IntoIter<String, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = (&'a String, &'a Attribute);
    type IntoIter = Iter<'a, String, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Attributes {
    type Item = (&'a String, &'a mut Attribute);
    type IntoIter = IterMut<'a, String, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl From<()> for Attributes {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<Vec<(&str, Attribute)>> for Attributes {
    fn from(from: Vec<(&str, Attribute)>) -> Self {
        let mut attrs = Attributes::new();

        for (name, attr) in from {
            attrs.set(name, attr);
        }

        attrs
    }
}
