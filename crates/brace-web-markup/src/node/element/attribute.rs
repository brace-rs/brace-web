use indexmap::map::{IndexMap, IntoIter, Iter, IterMut};

#[derive(Clone, Default)]
pub struct Attrs(IndexMap<String, String>);

impl Attrs {
    pub fn new() -> Self {
        Self(IndexMap::new())
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
        self.0.swap_remove(key.as_ref());
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
