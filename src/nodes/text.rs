use std::fmt::Debug;
use super::node::Node;


#[derive(Clone)]
pub struct Text {
    pub val: String,
    pub dom_text_node: Option<web_sys::Text>,
}

impl Text {
    pub fn new(val: impl ToString) -> Self {
        Self {
            val: val.to_string(),
            dom_text_node: None,
        }
    }
    pub fn node<M>(self) -> Node<M> {
        return Node::Text(self);
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.val.fmt(f)
    }
}