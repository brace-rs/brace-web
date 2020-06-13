use std::fmt::Write;

use futures::future::{self, Ready};
use serde::{Deserialize, Serialize};

use brace_web_core::{HttpRequest, HttpResponse, Responder};

use crate::util::render::{render, Error, Render, Renderer, Result as RenderResult};
use crate::{Node, Nodes};

pub fn document() -> Document {
    Document::new()
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Document {
    nodes: Nodes,
}

impl Document {
    pub fn new() -> Self {
        Self {
            nodes: Nodes::new(),
        }
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

impl Render for Document {
    fn render(&self, renderer: &mut Renderer) -> RenderResult {
        write!(renderer, "<!DOCTYPE html>")?;

        for node in &self.nodes {
            node.render(renderer)?;
        }

        Ok(())
    }
}

impl Responder for Document {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        match render(&self) {
            Ok(body) => future::ok(
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(body),
            ),
            Err(err) => future::err(err),
        }
    }
}

impl From<Node> for Document {
    fn from(node: Node) -> Self {
        Self::new().with_node(node)
    }
}

impl From<Nodes> for Document {
    fn from(nodes: Nodes) -> Self {
        Self::new().with_nodes(nodes)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Document, Element, Text};

    #[test]
    fn test_document_tree() {
        let document = Document::new().with_node(
            Element::new("html")
                .with_node(Element::new("body").with_node(Text::new("hello world"))),
        );

        assert_eq!(document.nodes().len(), 1);

        let html = document.nodes().get(0).unwrap();

        assert_eq!(html.as_element().unwrap().nodes().len(), 1);

        let body = html.as_element().unwrap().nodes().get(0).unwrap();

        assert_eq!(body.as_element().unwrap().nodes().len(), 1);

        let text = body.as_element().unwrap().nodes().get(0).unwrap();

        assert_eq!(text.as_text().unwrap().value(), "hello world");
    }
}
