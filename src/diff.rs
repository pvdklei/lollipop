//! diff.rs finds the differences between the old
//! and the new virtual dom, and updates the real dom
//! to these changes.

use crate::{
    console_log, draw,
    events::ListenerKind::{self, Evented, Factory, Valued},
    nodes::{element::Element, node::Node, text::Text, widget::Widget},
    patch,
};

pub fn diff_node<M>(wname: &str, from: &mut Node<M>, to: &mut Node<M>) {
    match (from, to) {
        (Node::Element(from), Node::Element(to)) => diff_element(wname, from, to),
        (Node::Text(from), Node::Text(to)) => diff_text(from, to),
        (Node::Widget(from), Node::Widget(to)) => diff_widget(from, to),
        (from, to) => {
            draw::node(to);
            patch::replace_node(from.dom_node().unwrap(), to.dom_node().unwrap())
        }
    }
}

pub fn diff_widget(from: &Widget, to: &Widget) {
    if from.name != to.name {
        patch::replace_node(&from.dom_node, &to.dom_node);
    }
}

pub fn diff_text(from: &mut Text, to: &mut Text) {
    match from.dom_text_node.take() {
        Some(text_node) => {
            if from.val != to.val {
                patch::set_text_data(&text_node, &to.val);
            }
            to.dom_text_node = Some(text_node);
        }
        None => todo!(),
    }
}

pub fn diff_element<M>(wname: &str, from: &mut Element<M>, to: &mut Element<M>) {
    match from.dom_element.take() {
        Some(dom_element) => {
            if diff_name(&dom_element, from, to) {
                diff_attributes(&dom_element, from, to);
                diff_children(wname, &dom_element, from, to);
                diff_listeners(wname, &dom_element, from, to)
            }
            to.dom_element = Some(dom_element);
        }
        None => todo!(),
    }
}

fn diff_name<M>(dom_element: &web_sys::Element, from: &Element<M>, to: &mut Element<M>) -> bool {
    if from.tag != to.tag {
        draw::element(to);

        // unwrapping is safe because 'draw::element()' must have provided
        patch::replace_node(&dom_element, to.dom_element.as_ref().unwrap());
        false
    } else {
        true
    }
}

fn diff_attributes<M>(dom_element: &web_sys::Element, from: &Element<M>, to: &Element<M>) {
    for (from_name, from_val) in &from.attrs {
        match to.attrs.get(from_name) {
            Some(to_val) => {
                if from_val != to_val {
                    patch::set_attribute(dom_element, from_name, to_val);
                }
            }
            None => patch::remove_attribute(dom_element, from_name),
        }
    }
    for (to_name, to_val) in &to.attrs {
        if !from.attrs.contains_key(to_name) {
            patch::set_attribute(dom_element, to_name, to_val)
        }
    }
}

fn diff_children<M>(
    wname: &str,
    dom_element: &web_sys::Element,
    from: &mut Element<M>,
    to: &mut Element<M>,
) {
    let from_len = from.children.len();
    let to_len = to.children.len();

    if from_len >= to_len {
        patch::remove_children(dom_element, from_len - to_len)
    } else {
        patch::add_children(
            dom_element,
            to.children[from_len..].iter_mut().map(|child| {
                draw::node(child);

                // unwrapping is safe because 'draw::element()' must have provided
                child.dom_node().unwrap()
            }),
        )
    }
    for i in 0..std::cmp::min(from_len, to_len) {
        diff_node(wname, &mut from.children[i], &mut to.children[i]);
    }
}

fn diff_listeners<M>(
    wname: &str,
    dom_element: &web_sys::Element,
    from: &mut Element<M>,
    to: &mut Element<M>,
) {
    for (from_name, from_val) in from.listeners.iter() {
        match to.listeners.get_mut(from_name) {
            Some(to_val) => {
                if listener_are_different(&from_val.kind, &to_val.kind) {
                    console_log!("Listeners are different!");
                    patch::set_listener(wname, dom_element, from_name, to_val);
                }
            }
            None => {
                patch::remove_listener(dom_element, from_name, from_val);
            }
        }
    }
    for (to_name, to_val) in to.listeners.iter_mut() {
        if !from.listeners.contains_key(to_name) {
            patch::set_listener(wname, dom_element, to_name, to_val)
        }
    }
}

pub fn listener_are_different<M>(l1: &ListenerKind<M>, l2: &ListenerKind<M>) -> bool {
    match (l1, l2) {
        (Factory(fac1), Factory(fac2)) => fac1 != fac2, 
        (Evented(cb1), Evented(cb2)) => cb1 != cb2,
        (Valued(cb1), Valued(cb2)) => cb1 != cb2, 
        (_, _) => true,
    }
}

// #[cfg(test)]
// pub mod tests {
//     use std::{clone, time::Duration};

//     use crate::{nodes::Element, widget::{self, Widget}};
//     use wasm_bindgen_test::*;
//     wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

//     #[wasm_bindgen_test]
//     fn draw_tree() {
//         let body = crate::utils::document().body().unwrap();

//         let original = Element::<()>::new("div")
//             .attr("style", "background-color:lightblue;")
//             .child(Element::new("h3").child("yooo"))
//             .child(Element::new("h1").child("Testje"))
//             .child("Klein dom boompje")
//             .draw()
//             .attach(&body);

//         let mut copy = original.clone().draw().attach(&body);

//         let mut updated = Element::<()>::new("div")
//             .attr("style", "background-color:powderblue;")
//             .child(Element::new("h1").child("Testje erna"))
//             .child("Groot dom boompje")
//             .child(
//                 Element::new("div")
//                     .child("chilyoood")
//                     .child(Element::new("button").child("click")),
//             )
//             .child("onder")
//             .draw()
//             .attach(&body);

//         let mut w = Widget::dummy();

//         crate::diff::diff_element(&mut w, &mut copy, &mut updated);
//     }
// }
