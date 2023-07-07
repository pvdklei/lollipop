use super::node::Node;
use crate::events::{Listener, ListenerKind};
use std::collections::HashMap;
use std::fmt::Debug;

pub struct Element<M: 'static> {
    pub tag: &'static str,
    pub attrs: HashMap<&'static str, String>,
    pub children: Vec<Node<M>>,
    pub dom_element: Option<web_sys::Element>,
    pub listeners: HashMap<&'static str, Listener<M>>,
}

impl<M: Debug> Debug for Element<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut x = &mut f.debug_struct(self.tag);

        for (name, value) in self.attrs.iter() {
            x = x.field(name, value);
        }

        for (name, listener) in self.listeners.iter() {
            x = x.field(name, &listener.kind)
        }

        x.field("children", &self.children)
            .field("dom", &self.dom_element.is_some())
            .finish()
    }
}

impl<M> Element<M> {
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag,
            attrs: HashMap::new(),
            children: Vec::new(),
            dom_element: None,
            listeners: HashMap::new(),
        }
    }

    pub fn child(mut self, child: impl Into<Node<M>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn attr(mut self, name: &'static str, val: impl ToString) -> Self {
        self.attrs.insert(name, val.to_string());
        self
    }

    pub fn on(self, event: &'static str, action: fn() -> M) -> Self {
        self.on_listener_kind(event, action.into())
    }

    pub fn on_value(self, event: &'static str, action: fn(String) -> M) -> Self {
        self.on_listener_kind(event, action.into())
    }

    pub fn on_event(self, event: &'static str, action: fn(web_sys::Event) -> M) -> Self {
        self.on_listener_kind(event, action.into())
    }

    pub fn on_listener_kind(mut self, event: &'static str, action: ListenerKind<M>) -> Self {
        self.listeners.insert(
            event,
            Listener {
                kind: action,
                callback: None,
            },
        );
        self
    }

    pub fn node(self) -> Node<M> {
        return Node::Element(self);
    }

}


