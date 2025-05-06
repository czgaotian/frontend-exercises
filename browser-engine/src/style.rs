use std::collections::HashMap;

use crate::css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value};
use crate::dom::{ElementData, Node, NodeType};

type PropertyMap = HashMap<String, Value>;

pub struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyledNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match selector {
        Selector::Simple(s) => matches_simple_selector(elem, s),
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // check type selector
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // check ID selector
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // check class selectors
    if selector
        .class
        .iter()
        .any(|class| !elem.classes().contains(class.as_str()))
    {
        return false;
    }

    return true;
}

type MatchedRule<'a> = (Specificity, &'a Rule);

// If `rule` matches `elem`, return a `MatchedRule`. Otherwise return `None`.
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Go through the rules from lowest to highest specificity
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => HashMap::new(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::Declaration;
    use crate::dom::{elem, text};

    #[test]
    fn test_match_simple_selector() {
        let element = ElementData {
            tag_name: "div".to_string(),
            attrs: HashMap::new(),
        };
        let selector = SimpleSelector {
            tag_name: Some("div".to_string()),
            id: None,
            class: vec![],
        };
        assert!(matches_simple_selector(&element, &selector));
    }

    #[test]
    fn test_specified_values() {
        let mut stylesheet = Stylesheet { rules: vec![] };
        let declarations = vec![
            Declaration {
                name: "color".to_string(),
                value: Value::Keyword("red".to_string()),
            },
            Declaration {
                name: "font-size".to_string(),
                value: Value::Keyword("16px".to_string()),
            },
        ];
        let rule = Rule {
            selectors: vec![Selector::Simple(SimpleSelector {
                tag_name: Some("p".to_string()),
                id: None,
                class: vec![],
            })],
            declarations,
        };
        stylesheet.rules.push(rule);
        let element = ElementData {
            tag_name: "p".to_string(),
            attrs: HashMap::new(),
        };
        let values = specified_values(&element, &stylesheet);
        assert_eq!(
            values.get("color"),
            Some(&Value::Keyword("red".to_string()))
        );
        assert_eq!(
            values.get("font-size"),
            Some(&Value::Keyword("16px".to_string()))
        );
    }

    #[test]
    fn test_style_tree() {
        let mut stylesheet = Stylesheet { rules: vec![] };
        let declarations = vec![Declaration {
            name: "display".to_string(),
            value: Value::Keyword("block".to_string()),
        }];
        let rule = Rule {
            selectors: vec![Selector::Simple(SimpleSelector {
                tag_name: Some("div".to_string()),
                id: None,
                class: vec![],
            })],
            declarations,
        };
        stylesheet.rules.push(rule);
        let root = elem(
            "div".to_string(),
            HashMap::new(),
            vec![text("Hello".to_string())],
        );
        let styled_tree = style_tree(&root, &stylesheet);
        assert_eq!(
            styled_tree.value("display"),
            Some(Value::Keyword("block".to_string()))
        );
    }
}
