pub use crate::node::element::Element;
pub use crate::node::text::Text;
pub use crate::node::Node;

pub mod node;

#[cfg(test)]
mod tests {
    use crate::{Element, Node, Text};

    #[test]
    fn test_node_tree() {
        let mut html = Element::new("html");
        let mut body = Element::new("body");
        let text = Text::new("Hello world");

        body.nodes.push(Node::Text(text));

        assert_eq!(body.nodes.len(), 1);

        html.nodes.push(Node::Element(body));

        assert_eq!(html.nodes.len(), 1);
    }

    #[test]
    fn test_node_tree_alternative() {
        let mut html = Node::element("html");
        let mut body = Node::element("body");
        let text = Node::text("hello world");

        body.as_element_mut().unwrap().nodes.push(text);

        assert_eq!(body.as_element_mut().unwrap().nodes.len(), 1);

        html.as_element_mut().unwrap().nodes.push(body);

        assert_eq!(html.as_element_mut().unwrap().nodes.len(), 1);
    }
}
