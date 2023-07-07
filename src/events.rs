use std::fmt::Debug;

use crate::nodes::{element::Element, node::Node, update_widget};
use wasm_bindgen::{prelude::Closure, JsCast};

pub fn listen<M>(wname: &str, node: &mut Node<M>)
where
    M: 'static,
{
    match node {
        Node::Element(Element {
            listeners,
            children,
            dom_element: Some(dom_element),
            ..
        }) => {
            for (name, listener) in listeners.iter_mut() {
                add_event_listener(wname, name, listener, dom_element);
            }
            for child in children {
                listen(wname, child)
            }
        }
        Node::Element(Element {
            dom_element: None, ..
        }) => todo!(),
        Node::Text(_) | Node::Widget(_) => (),
    }
}

pub fn add_event_listener<M>(
    wname: &str,
    name: &'static str,
    listener: &mut Listener<M>,
    dom_element: &web_sys::Element,
) where
    M: 'static,
{
    let wname = wname.to_string();
    let cb = match listener.kind {
        Factory(fac) => Closure::wrap(Box::new(move || {
            let msg = fac();
            update_widget(&wname, msg);
        }) as Box<dyn FnMut()>)
        .into_js_value()
        .unchecked_into(),

        Evented(cb) => Closure::wrap(Box::new(move |e: web_sys::Event| {
            let msg = cb(e);
            update_widget(&wname, msg);
        }) as Box<dyn FnMut(web_sys::Event)>)
        .into_js_value()
        .unchecked_into(),

        Valued(cb) => Closure::wrap(Box::new(move |e: web_sys::Event| {
            let msg = cb(e
                .target()
                .unwrap()
                .dyn_ref::<web_sys::Element>()
                .unwrap()
                .node_value()
                .expect("This node doesn't have a value!"));
            update_widget(&wname, msg);
        }) as Box<dyn FnMut(web_sys::Event)>)
        .into_js_value()
        .unchecked_into(),

    };

    match dom_element.add_event_listener_with_callback(name, &cb) {
        Ok(()) => (),
        Err(_) => todo!(),
    };

    listener.callback = Some(cb)
}

#[derive(Clone, Debug)]
pub struct Listener<M: 'static> {
    pub kind: ListenerKind<M>,
    pub callback: Option<js_sys::Function>,
}

#[derive(Clone, Debug)]
pub enum ListenerKind<M: 'static> {
    Factory(fn() -> M),
    Evented(fn(web_sys::Event) -> M),
    Valued(fn(String) -> M),
}
pub use ListenerKind::*;

impl<M> From<fn() -> M> for ListenerKind<M> {
    fn from(x: fn() -> M) -> Self {
        Factory(x)
    }
}

impl<M> From<fn(web_sys::Event) -> M> for ListenerKind<M> {
    fn from(x: fn(web_sys::Event) -> M) -> Self {
        Evented(x)
    }
}

impl<M> From<fn(String) -> M> for ListenerKind<M> {
    fn from(x: fn(String) -> M) -> Self {
        Valued(x)
    }
}


