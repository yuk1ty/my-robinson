use std::collections::HashMap;

use crate::{
    css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value},
    dom::{ElementData, Node, NodeType},
};

type PropertyMap = HashMap<String, Value>;

#[derive(Debug, PartialEq)]
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
        self.specified_values.get(name).map(|v| v.clone())
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

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    values
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    let elem_classes = elem.classes();
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    }

    true
}

#[cfg(test)]
mod test {
    use crate::{
        css::{self, Color},
        html,
    };

    use super::*;

    #[test]
    fn test_style() {
        let html_node = html::parse(r#"<h1 class="test">head line</h1>"#.to_string());
        let stylesheet = css::parse(r#".test { color: #000000; }"#.to_string());
        let actual = style_tree(&html_node, &stylesheet);

        assert_eq!(
            actual,
            StyledNode {
                node: &Node {
                    children: vec![Node {
                        children: vec![],
                        node_type: NodeType::Text("head line".to_string())
                    }],
                    node_type: NodeType::Element(ElementData {
                        tag_name: "h1".to_string(),
                        attributes: {
                            let mut ret = HashMap::new();
                            ret.insert("class".to_string(), "test".to_string());
                            ret
                        }
                    })
                },
                specified_values: {
                    let mut ret = HashMap::new();
                    ret.insert(
                        "color".to_string(),
                        Value::ColorValue(Color {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: 255,
                        }),
                    );
                    ret
                },
                children: vec![StyledNode {
                    node: &Node {
                        children: vec![],
                        node_type: NodeType::Text("head line".to_string()),
                    },
                    specified_values: HashMap::new(),
                    children: vec![]
                }]
            }
        );
    }
}
