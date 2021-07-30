use std::collections::HashMap;

use crate::{css::Value, dom::Node};

type PropertyMap = HashMap<String, Value>;

struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}
