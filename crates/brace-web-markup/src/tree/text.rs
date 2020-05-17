use std::fmt::Write;

use crate::util::render::{Render, Renderer, Result as RenderResult};

pub fn text<T>(text: T) -> Text
where
    T: Into<String>,
{
    Text::new(text)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Text(String);

impl Text {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self(text.into())
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

impl From<&str> for Text {
    fn from(from: &str) -> Self {
        Self(from.to_owned())
    }
}

impl From<String> for Text {
    fn from(from: String) -> Self {
        Self(from)
    }
}
