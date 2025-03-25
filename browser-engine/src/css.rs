struct Stylesheet {
    rules: Vec<Rule>,
}

struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

enum Selector {
    Simple(SimpleSelector),
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

struct Declaration {
    name: String,
    value: Value,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

enum Unit {
    Px,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser {
        pos: 0,
        input: source,
    };
    Stylesheet {
        rules: parser.parse_rules(),
    }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// If the exact string `s` is found at the current position, consume it.
    /// Otherwise, panic.
    fn expect_char(&mut self, c: char) {
        if self.consume_char() != c {
            panic!("Expected {:?} at byte {} but it was not found", c, self.pos);
        }
    }

    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }

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

    /// Parse a list of rule sets, separated by optional whitespace.
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            rules.push(self.parse_rule());
        }
        rules
    }

    // Parse a rule set: `<selectors> { <declarations> }`.
    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    // Parse one simple selector, e.g.: `type#id.class1.class2.class3`
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    // universal selector
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }
        return selector;
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        self.expect_char('{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = self.parse_identifier();
        self.consume_whitespace();
        self.expect_char(':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        self.expect_char(';');

        Declaration { name, value }
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float(&mut self) -> f32 {
        self.consume_while(|c| matches!(c, '0'..='9' | '.'))
            .parse()
            .unwrap()
    }

    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit"),
        }
    }

    fn parse_color(&mut self) -> Value {
        self.expect_char('#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255,
        })
    }

    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos..self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }
}

fn valid_identifier_char(c: char) -> bool {
    // TODO: Include U+00A0 and higher.
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identifier() {
        let mut parser = Parser {
            pos: 0,
            input: "example".to_string(),
        };
        assert_eq!(parser.parse_identifier(), "example");
    }

    #[test]
    fn test_parse_simple_selector() {
        let mut parser = Parser {
            pos: 0,
            input: "div#main.container".to_string(),
        };
        let selector = parser.parse_simple_selector();
        assert_eq!(selector.tag_name, Some("div".to_string()));
        assert_eq!(selector.id, Some("main".to_string()));
        assert_eq!(selector.class, vec!["container".to_string()]);
    }

    #[test]
    fn test_parse_length() {
        let mut parser = Parser {
            pos: 0,
            input: "15px".to_string(),
        };
        if let Value::Length(value, unit) = parser.parse_length() {
            assert_eq!(value, 15.0);
            assert!(matches!(unit, Unit::Px));
        } else {
            panic!("Failed to parse length");
        }
    }

    #[test]
    fn test_parse_color() {
        let mut parser = Parser {
            pos: 0,
            input: "#ffcc00".to_string(),
        };
        if let Value::ColorValue(color) = parser.parse_color() {
            assert_eq!(color.r, 0xff);
            assert_eq!(color.g, 0xcc);
            assert_eq!(color.b, 0x00);
            assert_eq!(color.a, 255);
        } else {
            panic!("Failed to parse color");
        }
    }

    #[test]
    fn test_parse_declaration() {
        let mut parser = Parser {
            pos: 0,
            input: "color: red;".to_string(),
        };
        let decl = parser.parse_declaration();
        assert_eq!(decl.name, "color");
        if let Value::Keyword(value) = decl.value {
            assert_eq!(value, "red");
        } else {
            panic!("Failed to parse declaration");
        }
    }

    #[test]
    fn test_parse_rules() {
        let mut parser = Parser {
            pos: 0,
            input: "div { color: blue; width: 100px; }".to_string(),
        };
        let stylesheet = parse(parser.input.clone());
        assert_eq!(stylesheet.rules.len(), 1);
        let rule = &stylesheet.rules[0];
        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 2);
    }
}
