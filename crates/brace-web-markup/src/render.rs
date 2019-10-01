use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult, Write};
use std::result::Result as StdResult;

pub type Result = StdResult<(), Error>;

pub fn render<T>(item: T) -> StdResult<String, Error>
where
    T: Render,
{
    let mut buffer = String::new();
    let mut renderer = Renderer::new(&mut buffer);

    renderer.render(item)?;

    Ok(buffer)
}

pub trait Render {
    fn render(&self, renderer: &mut Renderer) -> Result;
}

pub struct Renderer<'a>(&'a mut (dyn Write + 'a));

impl<'a> Renderer<'a> {
    pub fn new<T>(buffer: &'a mut T) -> Self
    where
        T: Write,
    {
        Self(buffer)
    }

    pub fn render<T>(&mut self, item: T) -> Result
    where
        T: Render,
    {
        item.render(self)
    }
}

impl Write for Renderer<'_> {
    fn write_str(&mut self, s: &str) -> FmtResult {
        self.0.write_str(s)
    }
}

#[derive(Debug)]
pub enum Error {
    Message(String),
    Format(FmtError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Message(message) => write!(f, "{}", message),
            Self::Format(error) => write!(f, "{}", error),
        }
    }
}

impl StdError for Error {}

impl From<FmtError> for Error {
    fn from(from: FmtError) -> Self {
        Self::Format(from)
    }
}

#[cfg(test)]
mod tests {
    use super::render;
    use crate::{Element, Node};

    #[test]
    fn test_render_node() {
        let node_1 = Node::element("html");

        assert_eq!(render(node_1).unwrap(), "<html></html>");

        let mut node_2 = Node::element("html");

        node_2
            .as_element_mut()
            .unwrap()
            .attrs_mut()
            .insert("xmlns", "http://www.w3.org/1999/xhtml");

        node_2
            .as_element_mut()
            .unwrap()
            .nodes_mut()
            .append(Node::element(Element::with(
                "head",
                (),
                Element::with("title", (), "Hello world"),
            )))
            .append(Node::element(Element::with("body", (), "hello world")));

        assert_eq!(
            render(node_2).unwrap(),
            "<html xmlns=\"http://www.w3.org/1999/xhtml\"><head><title>Hello world</title></head><body>hello world</body></html>"
        );

        let mut node_3 = Node::element("div");

        node_3
            .as_element_mut()
            .unwrap()
            .attrs_mut()
            .insert("b", "1")
            .insert("a", "2")
            .insert("c", "3");

        assert_eq!(
            render(node_3).unwrap(),
            "<div b=\"1\" a=\"2\" c=\"3\"></div>"
        );
    }
}
