use super::{element::Element, text::Text, widget::Widget};
use std::fmt::Debug;

pub enum Node<M: 'static> {
    Element(Element<M>),
    Text(Text),
    Widget(Widget),
}

impl<M: Debug> Debug for Node<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element(el) => el.fmt(f),
            Self::Text(txt) => txt.fmt(f),
            Self::Widget(widget) => widget.fmt(f),
        }
    }
}

impl<M> Node<M> {
    pub fn dom_node(&self) -> Option<&web_sys::Node> {
        match self {
            Self::Element(Element { dom_element, .. }) => {
                dom_element.as_ref().map(|el| el.as_ref())
            }
            Self::Text(Text { dom_text_node, .. }) => {
                dom_text_node.as_ref().map(|txt| txt.as_ref())
            }
            Self::Widget(widget) => Some(&widget.dom_node),
        }
    }

    pub fn element(&mut self) -> &mut Element<M> {
        match self {
            Self::Element(el) => el,
            _ => todo!(),
        }
    }

    pub fn attach(self, to: &web_sys::Node) -> Self {
        self.attach_borrow(to);
        self
    }

    pub fn attach_borrow(&self, to: &web_sys::Node) {
        let node: &web_sys::Node = self.dom_node().unwrap();
        match to.append_child(node) {
            Ok(_appended_child) => (),
            Err(_) => todo!(),
        };
    }
}

impl<M> From<Element<M>> for Node<M> {
    fn from(el: Element<M>) -> Self {
        Self::Element(el)
    }
}

impl<M> From<Text> for Node<M> {
    fn from(txt: Text) -> Self {
        Self::Text(txt)
    }
}

impl<M> From<Widget> for Node<M> {
    fn from(w: Widget) -> Self {
        Self::Widget(w)
    }
}

impl<T, M> From<T> for Node<M>
where
    T: Into<String>,
{
    fn from(data: T) -> Self {
        Self::Text(Text::new(data.into()))
    }
}
