use std::fmt::{self, Display, Write};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::util::render::{Render, Renderer, Result as RenderResult};

static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());

pub fn text<T>(text: T) -> Text
where
    T: AsRef<str>,
{
    Text::new(text)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Text(String);

impl Text {
    pub fn new<T>(text: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(REGEX.replace_all(text.as_ref(), " ").trim().to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn value_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl Render for Text {
    fn render(&self, renderer: &mut Renderer) -> RenderResult {
        Ok(write!(renderer, "{}", self.0)?)
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<&str> for Text {
    fn from(from: &str) -> Self {
        Self::new(from)
    }
}

impl From<String> for Text {
    fn from(from: String) -> Self {
        Self::new(from)
    }
}

#[cfg(test)]
mod tests {
    use super::Text;

    #[test]
    fn test_whitespace() {
        let a = Text::new("hello world");
        let b = Text::new("hello  world");
        let c = Text::new("hello   world");
        let d = Text::new("\n hello  \n  world \n");

        assert_eq!(a.0, "hello world");
        assert_eq!(b.0, "hello world");
        assert_eq!(c.0, "hello world");
        assert_eq!(d.0, "hello world");
    }
}
