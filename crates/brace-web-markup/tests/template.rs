use brace_parser::parser::parse;
use brace_web_markup::util::parser::document;
use brace_web_markup::{Element, Node, Text};

#[test]
fn test_template_1() {
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
        Sed eget nunc rhoncus velit pretium viverra. Pellentesque tempor lacus
        non diam convallis fermentum. Cras eu purus et massa tincidunt rhoncus
        eget ut lectus. Nulla lacus lorem, consequat quis pharetra at, gravida
        in turpis. Nullam iaculis dui ut felis pretium euismod. In erat mauris,
        volutpat vel augue vel, finibus eleifend dolor. Sed sodales porta
        ligula, vitae volutpat nulla.";

    assert_eq!(
        parse(include_str!("../fixtures/template-1.txt"), document),
        Ok((
            Node::element(
                Element::new("html")
                    .with_attr("lang", "en")
                    .with_nodes(vec![
                        Element::new("head")
                            .with_nodes(vec![
                                Element::new("meta").with_attr("charset", "utf-8").into(),
                                Element::new("title")
                                    .with_node(Text::new("Example 001"))
                                    .into(),
                                Element::new("meta")
                                    .with_attr("name", "description")
                                    .with_attr("content", "Example 001.")
                                    .into(),
                                Element::new("meta")
                                    .with_attr("name", "author")
                                    .with_attr("content", "Me")
                                    .into(),
                                Element::new("link")
                                    .with_attr("rel", "stylesheet")
                                    .with_attr("href", "/assets/css/style.css")
                                    .into(),
                                Element::new("script")
                                    .with_attr("src", "/assets/js/script.js")
                                    .into(),
                            ])
                            .into(),
                        Element::new("body")
                            .with_nodes(vec![
                                Element::new("header")
                                    .with_node(
                                        Element::new("h1").with_node(Text::new("Example 001"))
                                    )
                                    .into(),
                                Element::new("main")
                                    .with_nodes(vec![
                                        Element::new("p")
                                            .with_node(Text::new("This is example 001."))
                                            .into(),
                                        Element::new("p").with_node(Text::new(lorem)).into(),
                                    ])
                                    .into(),
                            ])
                            .into(),
                    ]),
            )
            .into(),
            ""
        )),
    );
}

#[test]
fn test_template_2() {
    assert_eq!(
        parse(include_str!("../fixtures/template-2.txt"), document),
        Ok((
            Node::element(
                Element::new("div")
                    .with_attr("class", "field field--checkbox")
                    .with_nodes(vec![
                        Element::new("label")
                            .with_attr("for", "my-checkbox")
                            .with_node(Text::new("Checkbox"))
                            .into(),
                        Element::new("input")
                            .with_attr("id", "my-checkbox")
                            .with_attr("class", "input input--checkbox")
                            .with_attr("type", "checkbox")
                            .with_attr("checked", true)
                            .into(),
                        Element::new("span")
                            .with_attr("class", "description")
                            .with_node(Text::new("Description."))
                            .into(),
                    ]),
            )
            .into(),
            ""
        )),
    );
}

#[test]
fn test_template_3() {
    assert_eq!(
        parse(include_str!("../fixtures/template-3.txt"), document),
        Ok((
            Node::element(Element::new("div").with_node("hello world")).into(),
            ""
        ))
    );
}
