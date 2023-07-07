use crate::{
    console_log,
    events::{add_event_listener, Listener},
};

pub fn replace_node(dom_node: &web_sys::Node, by: &web_sys::Node) {
    console_log!(
        "[patch] replace node {}: {} by {}: {}",
        dom_node.node_name(),
        dom_node.node_type(),
        by.node_name(),
        by.node_name()
    );
    match dom_node.parent_node() {
        Some(parent) => match parent.replace_child(by, dom_node) {
            Ok(_replaced_node) => (),
            Err(_) => todo!(),
        },
        None => todo!(),
    }
}

pub fn set_attribute(dom_element: &web_sys::Element, name: &str, val: &str) {
    console_log!(
        "[patch] set attribute {} -> {} on {}",
        name,
        val,
        dom_element.tag_name()
    );
    match dom_element.set_attribute(name, val) {
        Ok(()) => (),
        Err(_) => todo!(),
    }
}

pub fn remove_attribute(dom_element: &web_sys::Element, name: &str) {
    console_log!(
        "[patch] remove attribute {} from {}",
        name,
        dom_element.tag_name()
    );
    match dom_element.remove_attribute(name) {
        Ok(()) => (),
        Err(_) => todo!(),
    }
}

pub fn set_listener<M>(
    wname: &str,
    dom_element: &web_sys::Element,
    name: &'static str,
    listener: &mut Listener<M>,
) where
    M: 'static,
{
    console_log!(
        "[patch] added listener {} to {}",
        name,
        dom_element.tag_name()
    );
    add_event_listener(wname, name, listener, dom_element)
}

pub fn remove_listener<M>(dom_element: &web_sys::Element, name: &str, listener: &Listener<M>) {
    console_log!(
        "[patch] removed listener {} from {}",
        name,
        dom_element.tag_name()
    );
    match dom_element.remove_event_listener_with_callback(name, listener.callback.as_ref().unwrap())
    {
        Ok(()) => (),
        Err(_) => todo!(),
    }
}

pub fn add_children<'a>(
    dom_element: &web_sys::Element,
    children: impl Iterator<Item = &'a web_sys::Node>,
) {
    let mut count = 0;
    for child in children {
        count += 1;
        match dom_element.append_child(child) {
            Ok(_appended_child) => (),
            Err(_) => todo!(),
        }
    }
    if count > 0 {
        console_log!(
            "[patch] add {} children to {}",
            count,
            dom_element.tag_name()
        );
    }
}

pub fn remove_children(dom_element: &web_sys::Element, n: usize) {
    if n > 0 {
        console_log!(
            "[patch] remove {} children from {}",
            n,
            dom_element.tag_name()
        );
    }

    for _ in 0..n {
        match dom_element.last_element_child() {
            Some(last_child) => last_child.remove(),
            None => todo!(),
        }
    }
}

pub fn set_text_data(dom_text: &web_sys::Text, data: &str) {
    console_log!(
        "[patch] set text data from {} to {}",
        dom_text.node_value().unwrap(),
        data
    );
    dom_text.set_data(data);
}
