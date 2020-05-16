use brace_parser::prelude::*;

use crate::node::attribute::{Attribute, Attributes};
use crate::node::element::Element;
use crate::node::text::Text;
use crate::node::{Node, Nodes};

pub fn document(input: &str) -> Output<Nodes> {
    parse(
        input,
        context(
            "document",
            delimited(
                optional(sequence::whitespace),
                map(optional(nodes), Option::unwrap_or_default),
                optional(sequence::whitespace),
            ),
        ),
    )
}

pub fn newline(input: &str) -> Output<&str> {
    parse(
        input,
        consume((
            optional(sequence::indent),
            sequence::linebreak,
            optional(sequence::indent),
        )),
    )
}

pub fn string(input: &str) -> Output<String> {
    parse(
        input,
        context(
            "string",
            delimited(
                '"',
                map(
                    optional(unescape(
                        escaped(
                            not(either('"', character::linebreak)),
                            branch(('"', '\\', 'n', 't', 'r', 'f')),
                        ),
                        branch((
                            '"',
                            '\\',
                            map('n', |_| '\n'),
                            map('t', |_| '\t'),
                            map('r', |_| '\r'),
                            map('f', |_| '\u{000C}'),
                        )),
                    )),
                    Option::unwrap_or_default,
                ),
                fail('"'),
            ),
        ),
    )
}

pub fn boolean(input: &str) -> Output<bool> {
    parse(
        input,
        context(
            "boolean",
            either(map("true", |_| true), map("false", |_| false)),
        ),
    )
}

pub fn node(input: &str) -> Output<Node> {
    parse(
        input,
        context(
            "node",
            either(map(text, Node::text), map(element, Node::element)),
        ),
    )
}

pub fn nodes(input: &str) -> Output<Nodes> {
    parse(
        input,
        context("nodes", map(list(node, newline), Nodes::from)),
    )
}

pub fn text(input: &str) -> Output<Text> {
    parse(
        input,
        context(
            "text",
            map(
                delimited(
                    '"',
                    map(
                        optional(unescape(
                            escaped(not('"'), branch(('"', '\\'))),
                            branch(('"', '\\')),
                        )),
                        Option::unwrap_or_default,
                    ),
                    fail('"'),
                ),
                Text::from,
            ),
        ),
    )
}

pub fn element(input: &str) -> Output<Element> {
    parse(
        input,
        context(
            "element",
            map(
                trio(
                    tag,
                    map(
                        optional(leading(optional(sequence::indent), attributes)),
                        Option::unwrap_or_default,
                    ),
                    map(
                        optional(leading(optional(sequence::indent), body)),
                        Option::unwrap_or_default,
                    ),
                ),
                Element::from,
            ),
        ),
    )
}

pub fn tag(input: &str) -> Output<&str> {
    parse(
        input,
        context(
            "tag",
            trailing(
                consume(list(
                    (sequence::alphabetic, optional(sequence::alphanumeric)),
                    '-',
                )),
                fail(peek(either(sequence::whitespace, end))),
            ),
        ),
    )
}

pub fn body(input: &str) -> Output<Nodes> {
    parse(
        input,
        context(
            "body",
            either(
                leading(
                    '|',
                    fail(leading(optional(sequence::indent), map(node, Nodes::from))),
                ),
                delimited(
                    '{',
                    fail(delimited(
                        optional(sequence::whitespace),
                        map(optional(nodes), Option::unwrap_or_default),
                        optional(sequence::whitespace),
                    )),
                    fail('}'),
                ),
            ),
        ),
    )
}

pub fn key(input: &str) -> Output<&str> {
    parse(
        input,
        context("key", consume(list(sequence::alphabetic, '-'))),
    )
}

pub fn attribute(input: &str) -> Output<Attribute> {
    parse(
        input,
        context(
            "attribute",
            either(
                map(string, Attribute::string),
                map(boolean, Attribute::boolean),
            ),
        ),
    )
}

pub fn attributes(input: &str) -> Output<Attributes> {
    parse(
        input,
        context(
            "attributes",
            map(
                list(
                    pair(
                        key,
                        map(
                            optional(leading(
                                leading(optional(sequence::indent), '='),
                                fail(leading(optional(sequence::indent), attribute)),
                            )),
                            |attr| attr.unwrap_or_else(|| Attribute::boolean(true)),
                        ),
                    ),
                    (optional(sequence::indent), ',', optional(sequence::indent)),
                ),
                Attributes::from,
            ),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use brace_parser::sequence::Sequence;

    #[test]
    fn test_template() {
        let lorem = r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
                Sed eget nunc rhoncus velit pretium viverra. Pellentesque tempor
                lacus non diam convallis fermentum. Cras eu purus et massa
                tincidunt rhoncus eget ut lectus. Nulla lacus lorem, consequat
                quis pharetra at, gravida in turpis. Nullam iaculis dui ut felis
                pretium euismod. In erat mauris, volutpat vel augue vel, finibus
                eleifend dolor. Sed sodales porta ligula, vitae volutpat nulla."#;

        assert_eq!(
            parse(include_str!("../templates/example-001.txt"), document),
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
        assert_eq!(
            parse(include_str!("../templates/example-002.txt"), document),
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
    fn test_string() {
        assert_eq!(
            parse("", string),
            Err(Error::expect('"').but_found_end().with_context("string"))
        );
        assert_eq!(
            parse("hello", string),
            Err(Error::expect('"').but_found('h').with_context("string"))
        );
        assert_eq!(
            parse("\"", string),
            Err(Error::expect('"')
                .but_found_end()
                .with_context("string")
                .into_fail())
        );
        assert_eq!(parse("\"\"", string), Ok((String::new(), "")));
        assert_eq!(parse("\"hello\"", string), Ok((String::from("hello"), "")));
        assert_eq!(
            parse("\"hello world\"", string),
            Ok((String::from("hello world"), ""))
        );
        assert_eq!(
            parse("\"hello\" world", string),
            Ok((String::from("hello"), " world"))
        );
        assert_eq!(
            parse("\"hello\" world\"", string),
            Ok((String::from("hello"), " world\""))
        );
        assert_eq!(
            parse("\"hello world", string),
            Err(Error::expect('"')
                .but_found_end()
                .with_context("string")
                .into_fail())
        );
        assert_eq!(
            parse("hello world", string),
            Err(Error::expect('"').but_found('h').with_context("string"))
        );
        assert_eq!(
            parse("\"hello\nworld\"", string),
            Err(Error::expect('"')
                .but_found('\n')
                .with_context("string")
                .into_fail())
        );
        assert_eq!(
            parse("\"hello\rworld\"", string),
            Err(Error::expect('"')
                .but_found('\r')
                .with_context("string")
                .into_fail())
        );
        assert_eq!(
            parse("\"hello\x0Cworld\"", string),
            Err(Error::expect('"')
                .but_found('\x0C')
                .with_context("string")
                .into_fail())
        );
        assert_eq!(
            parse("\"hello\tworld\"", string),
            Ok((String::from("hello\tworld"), ""))
        );
        assert_eq!(
            parse("\"hello\\\\world\"", string),
            Ok((String::from("hello\\world"), ""))
        );
        assert_eq!(
            parse("\"hello\\tworld\"", string),
            Ok((String::from("hello\tworld"), ""))
        );
        assert_eq!(
            parse("\"hello\\nworld\"", string),
            Ok((String::from("hello\nworld"), ""))
        );
        assert_eq!(
            parse("\"hello\\rworld\"", string),
            Ok((String::from("hello\rworld"), ""))
        );
        assert_eq!(
            parse("\"hello\\fworld\"", string),
            Ok((String::from("hello\x0Cworld"), ""))
        );
        assert_eq!(
            parse("\"\\\"hello world\\\"\"", string),
            Ok((String::from("\"hello world\""), ""))
        );
        assert_eq!(
            parse("\"\\\"\\\\\\\"hello world\\\\\\\"\\\"\"", string),
            Ok((String::from("\"\\\"hello world\\\"\""), ""))
        );
        assert_eq!(
            parse(
                "\"\\\"\\\\\\\"\\\\\\\\\\\\\\\"hello world\\\\\\\\\\\\\\\"\\\\\\\"\\\"\"",
                string
            ),
            Ok((String::from("\"\\\"\\\\\\\"hello world\\\\\\\"\\\"\""), ""))
        );
    }

    #[test]
    fn test_boolean() {
        assert_eq!(
            parse("", boolean),
            Err(Error::expect('f').but_found_end().with_context("boolean"))
        );
        assert_eq!(parse("true", boolean), Ok((true, "")));
        assert_eq!(parse("false", boolean), Ok((false, "")));
        assert_eq!(
            parse("null", boolean),
            Err(Error::expect('f').but_found('n').with_context("boolean"))
        );
    }

    #[test]
    fn test_node() {
        assert_eq!(parse("element", node), Ok((Node::element("element"), "")));
        assert_eq!(parse("\"text\"", node), Ok((Node::text("text"), "")));
        assert_eq!(
            parse("div { span | \"text\" }", node),
            Ok((
                Element::new("div")
                    .with_node(Element::new("span").with_node(Text::new("text")))
                    .into(),
                ""
            ))
        );
        assert_eq!(
            parse("div { span$ | \"text\" }", node),
            Err(Error::expect(Expect::End)
                .but_found('$')
                .with_context("tag")
                .into_fail())
        );
    }

    #[test]
    fn test_nodes() {
        assert_eq!(
            parse("element", nodes),
            Ok((Node::element("element").into(), ""))
        );
        assert_eq!(
            parse("\"text\"", nodes),
            Ok((Node::text("text").into(), ""))
        );
        assert_eq!(
            parse("element \n element", nodes),
            Ok((
                vec![Node::element("element"), Node::element("element")].into(),
                ""
            ))
        );
        assert_eq!(
            parse("\"text\" \n \"text\"", nodes),
            Ok((vec![Node::text("text"), Node::text("text")].into(), ""))
        );
        assert_eq!(
            parse("div \n div", nodes),
            Ok((vec![Element::new("div"), Element::new("div")].into(), ""))
        );
        assert_eq!(
            parse("div {} \n div", nodes),
            Ok((vec![Element::new("div"), Element::new("div")].into(), ""))
        );
        assert_eq!(
            parse("div \n div {}", nodes),
            Ok((vec![Element::new("div"), Element::new("div")].into(), ""))
        );
        assert_eq!(
            parse("div {} \n div {}", nodes),
            Ok((vec![Element::new("div"), Element::new("div")].into(), ""))
        );
        assert_eq!(
            parse("div {} \n div { span | \"text\" }", nodes),
            Ok((
                vec![
                    Element::new("div"),
                    Element::new("div")
                        .with_node(Element::new("span").with_node(Text::new("text")))
                ]
                .into(),
                ""
            ))
        );
        assert_eq!(
            parse("div {} \n div { span$ | \"text\" }", nodes),
            Err(Error::expect(Expect::End)
                .but_found('$')
                .with_context("tag")
                .into_fail())
        );
    }

    #[test]
    fn test_text() {
        assert_eq!(
            parse("", text),
            Err(Error::expect('"').but_found_end().with_context("text"))
        );
        assert_eq!(
            parse("hello", text),
            Err(Error::expect('"').but_found('h').with_context("text"))
        );
        assert_eq!(
            parse("\"", text),
            Err(Error::expect('"')
                .but_found_end()
                .with_context("text")
                .into_fail())
        );
        assert_eq!(parse("\"\"", text), Ok((Text::from(""), "")));
        assert_eq!(parse("\"hello\"", text), Ok((Text::from("hello"), "")));
        assert_eq!(
            parse("\"hello world\"", text),
            Ok((Text::from("hello world"), ""))
        );
        assert_eq!(
            parse("\"hello\" world", text),
            Ok((Text::from("hello"), " world"))
        );
        assert_eq!(
            parse("\"hello\" world\"", text),
            Ok((Text::from("hello"), " world\""))
        );
        assert_eq!(
            parse("\"hello world", text),
            Err(Error::expect('"')
                .but_found_end()
                .with_context("text")
                .into_fail())
        );
        assert_eq!(
            parse("hello world", text),
            Err(Error::expect('"').but_found('h').with_context("text"))
        );
        assert_eq!(
            parse("\"hello\nworld\"", text),
            Ok((Text::from("hello\nworld"), ""))
        );
        assert_eq!(
            parse("\"hello\rworld\"", text),
            Ok((Text::from("hello\rworld"), ""))
        );
        assert_eq!(
            parse("\"hello\x0Cworld\"", text),
            Ok((Text::from("hello\x0cworld"), ""))
        );
        assert_eq!(
            parse("\"hello\tworld\"", text),
            Ok((Text::from("hello\tworld"), ""))
        );
        assert_eq!(
            parse("\"hello\\\\world\"", text),
            Ok((Text::from("hello\\world"), ""))
        );
        assert_eq!(
            parse("\"hello\\tworld\"", text),
            Err(Error::expect('"')
                .but_found('h')
                .with_context("text")
                .into_fail())
        );
        assert_eq!(
            parse("\"hello\\nworld\"", text),
            Err(Error::expect('"')
                .but_found('h')
                .with_context("text")
                .into_fail())
        );
        assert_eq!(
            parse("\"hello\\rworld\"", text),
            Err(Error::expect('"')
                .but_found('h')
                .with_context("text")
                .into_fail())
        );
        assert_eq!(
            parse("\"hello\\fworld\"", text),
            Err(Error::expect('"')
                .but_found('h')
                .with_context("text")
                .into_fail())
        );
        assert_eq!(
            parse("\"\\\"hello world\\\"\"", text),
            Ok((Text::from("\"hello world\""), ""))
        );
        assert_eq!(
            parse("\"\\\"\\\\\\\"hello world\\\\\\\"\\\"\"", text),
            Ok((Text::from("\"\\\"hello world\\\"\""), ""))
        );
        assert_eq!(
            parse(
                "\"\\\"\\\\\\\"\\\\\\\\\\\\\\\"hello world\\\\\\\\\\\\\\\"\\\\\\\"\\\"\"",
                text
            ),
            Ok((Text::from("\"\\\"\\\\\\\"hello world\\\\\\\"\\\"\""), ""))
        );
    }

    #[test]
    fn test_element() {
        assert_eq!(
            parse("", element),
            Err(Error::expect(Sequence::Alphabetic)
                .but_found_end()
                .with_context("tag"))
        );
        assert_eq!(parse("element", element), Ok((Element::new("element"), "")));
        assert_eq!(
            parse("element $", element),
            Ok((Element::new("element"), " $"))
        );
        assert_eq!(
            parse("element checked", element),
            Ok((Element::new("element").with_attr("checked", true), ""))
        );
        assert_eq!(
            parse("element class = \"custom\"", element),
            Ok((Element::new("element").with_attr("class", "custom"), ""))
        );
        assert_eq!(
            parse("element$", element),
            Err(Error::expect(Expect::End)
                .but_found('$')
                .with_context("tag")
                .into_fail())
        );
        assert_eq!(
            parse("div { span | h$ | \"Title\" }", element),
            Err(Error::expect(Expect::End)
                .but_found('$')
                .with_context("tag")
                .into_fail())
        );
    }

    #[test]
    fn test_tag() {
        assert_eq!(
            parse("", tag),
            Err(Error::expect(Sequence::Alphabetic)
                .but_found_end()
                .with_context("tag"))
        );
        assert_eq!(parse("custom", tag), Ok(("custom", "")));
        assert_eq!(parse("custom-element", tag), Ok(("custom-element", "")));
        assert_eq!(
            parse("custom-element-inner", tag),
            Ok(("custom-element-inner", ""))
        );
        assert_eq!(
            parse("custom--element", tag),
            Err(Error::expect(Expect::End)
                .but_found('-')
                .with_context("tag")
                .into_fail())
        );
        assert_eq!(
            parse("custom-element-", tag),
            Err(Error::expect(Expect::End)
                .but_found('-')
                .with_context("tag")
                .into_fail())
        );
        assert_eq!(
            parse("-element", tag),
            Err(Error::expect(sequence::Sequence::Alphabetic)
                .but_found('-')
                .with_context("tag"))
        );
    }

    #[test]
    fn test_body() {
        assert_eq!(
            parse("", body),
            Err(Error::expect('{').but_found_end().with_context("body"))
        );
        assert_eq!(
            parse("|", body),
            Err(Error::expect(Sequence::Alphabetic)
                .but_found_end()
                .with_context("tag")
                .into_fail())
        );
        assert_eq!(
            parse("|\n", body),
            Err(Error::expect(Sequence::Alphabetic)
                .but_found('\n')
                .with_context("tag")
                .into_fail())
        );
        assert_eq!(
            parse("{", body),
            Err(Error::expect('}')
                .but_found_end()
                .with_context("body")
                .into_fail())
        );
        assert_eq!(parse("{}", body), Ok((Nodes::new(), "")));
        assert_eq!(
            parse("| element", body),
            Ok((Node::element("element").into(), ""))
        );
        assert_eq!(
            parse("{ element }", body),
            Ok((Node::element("element").into(), ""))
        );
        assert_eq!(
            parse("{\n element \n}", body),
            Ok((Node::element("element").into(), ""))
        );
    }

    #[test]
    fn test_attribute() {
        assert_eq!(
            parse("", attribute),
            Err(Error::expect('f').but_found_end().with_context("boolean"))
        );
        assert_eq!(
            parse("hello world", attribute),
            Err(Error::expect('f').but_found('h').with_context("boolean"))
        );
        assert_eq!(
            parse("\"hello world\"", attribute),
            Ok((Attribute::string("hello world"), ""))
        );
        assert_eq!(parse("true", attribute), Ok((Attribute::boolean(true), "")));
        assert_eq!(
            parse("false", attribute),
            Ok((Attribute::boolean(false), ""))
        );
    }

    #[test]
    fn test_attributes() {
        assert_eq!(
            parse("one", attributes),
            Ok((
                {
                    let mut attrs = Attributes::new();
                    attrs.set("one", true);
                    attrs
                },
                ""
            ))
        );
        assert_eq!(
            parse("one, two", attributes),
            Ok((
                {
                    let mut attrs = Attributes::new();
                    attrs.set("one", true);
                    attrs.set("two", true);
                    attrs
                },
                ""
            ))
        );
        assert_eq!(
            parse("one = \"two\"", attributes),
            Ok((
                {
                    let mut attrs = Attributes::new();
                    attrs.set("one", "two");
                    attrs
                },
                ""
            ))
        );
        assert_eq!(
            parse("one = two", attributes),
            Err(Error::expect('f')
                .but_found('t')
                .with_context("boolean")
                .into_fail())
        );
        assert_eq!(
            parse(
                "one = \"hello\", two, three = true, four = false",
                attributes
            ),
            Ok((
                {
                    let mut attrs = Attributes::new();
                    attrs.set("one", "hello");
                    attrs.set("two", true);
                    attrs.set("three", true);
                    attrs.set("four", false);
                    attrs
                },
                ""
            ))
        );
    }
}
