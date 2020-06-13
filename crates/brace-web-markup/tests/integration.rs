use brace_web_markup::{body, document, em, html, text, Document};
use serde_json::{from_str, to_string_pretty};

#[test]
fn test_serde_integration() {
    let doc = document().with_node(
        html().with_node(
            body()
                .with_attr("greeting", true)
                .with_attr("message", "Hello world")
                .with_node(text("Hello"))
                .with_node(em().with_node(text("world"))),
        ),
    );

    let str_1 = to_string_pretty(&doc).unwrap();
    let str_2 = include_str!("../fixtures/serde.json");

    let doc_1: Document = from_str(&str_1).unwrap();
    let doc_2: Document = from_str(&str_2).unwrap();

    assert_eq!(doc_1, doc_2);
}
