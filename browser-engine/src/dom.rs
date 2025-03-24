use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Node {
    pub children: Vec<Node>,
    node_type: NodeType,
}

#[derive(PartialEq, Debug)]
enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(PartialEq, Debug)]
struct ElementData {
    tag_name: String,
    attrs: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node {
        children: vec![],
        node_type: NodeType::Text(data),
    }
}

pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData { tag_name, attrs }),
    }
}
