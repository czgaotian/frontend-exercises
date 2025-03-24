use std::collections::HashMap;

use crate::dom;

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.pos += s.len();
        } else {
            panic!("Expected {:?} at byte {} but it was not found", s, self.pos);
        }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }

    // Consume characters until `test` returns false.
    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' |'A'..='Z'| '0'..='9'))
    }

    // Parse a single node.
    fn parse_node(&mut self) -> dom::Node {
        if self.starts_with("<") {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    // Parse a text node.
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        // opening tag
        self.expect("<");
        let tag_name = self.parse_name();
        let attrs = self.parse_attributes();
        self.expect(">");

        // contents
        let children = self.parse_nodes();

        self.expect("</");
        self.expect(&tag_name);
        self.expect(">");

        dom::elem(tag_name, attrs, children)
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_name();
        self.expect("=");
        let value = self.parse_attr_value();
        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        let close_quote = self.consume_char();
        assert_eq!(open_quote, close_quote);
        value
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    pub fn parse(source: String) -> dom::Node {
        let mut nodes = Parser {
            pos: 0,
            input: source,
        }
        .parse_nodes();

        if nodes.len() == 1 {
            nodes.remove(0)
        } else {
            dom::elem("html".to_string(), HashMap::new(), nodes)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        dom::{elem, text},
        html::Parser,
    };

    #[test]
    fn test_parse_text() {
        let input = "Hello, world!".to_string();
        let result = super::Parser::parse(input);
        assert_eq!(result, text("Hello, world!".to_string()));
    }

    #[test]
    fn test_parse_single_element() {
        let input = "<div></div>".to_string();
        let result = Parser::parse(input);
        assert_eq!(result, elem("div".to_string(), HashMap::new(), vec![]));
    }

    #[test]
    fn test_parse_element_with_text() {
        let input = "<p>Hello</p>".to_string();
        let result = Parser::parse(input);
        assert_eq!(
            result,
            elem(
                "p".to_string(),
                HashMap::new(),
                vec![text("Hello".to_string())]
            )
        );
    }

    #[test]
    fn test_parse_element_with_attributes() {
        let input = "<img src=\"image.png\" alt=\"An image\"></img>".to_string();
        let result = Parser::parse(input);
        let mut attributes = HashMap::new();
        attributes.insert("src".to_string(), "image.png".to_string());
        attributes.insert("alt".to_string(), "An image".to_string());
        assert_eq!(result, elem("img".to_string(), attributes, vec![]));
    }

    #[test]
    fn test_parse_nested_elements() {
        let input = "<div><p>Hello</p><span>World</span></div>".to_string();
        let result = Parser::parse(input);
        assert_eq!(
            result,
            elem(
                "div".to_string(),
                HashMap::new(),
                vec![
                    elem(
                        "p".to_string(),
                        HashMap::new(),
                        vec![text("Hello".to_string())]
                    ),
                    elem(
                        "span".to_string(),
                        HashMap::new(),
                        vec![text("World".to_string())]
                    ),
                ],
            )
        );
    }
}
