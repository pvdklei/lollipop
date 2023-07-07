//! draw.rs handles rendering a virtual dom element to a real dom node.
//! basically, it fills in the dom node or dom element for every node

use crate::{
    nodes::{element::Element, node::Node, text::Text},
    utils::document,
};

pub fn node<M>(node: &mut Node<M>) {
    match node {
        Node::Element(el) => element(el),
        Node::Text(txt) => text(txt),
        Node::Widget(_) => {
            // widget is already drawn when created
        }
    }
}

pub fn text(text: &mut Text) {
    let dom_text_node = document().create_text_node(&text.val);
    text.dom_text_node = Some(dom_text_node);
}

pub fn element<M>(element: &mut Element<M>) {
    let dom_element = create_dom_element(
        element.tag,
        element.attrs.iter_mut().map(|(name, val)| (*name, &**val)),
        element.children.iter_mut(),
    );
    element.dom_element = Some(dom_element);
}

fn create_dom_element<'a, M: 'static>(
    name: &str,
    attrs: impl Iterator<Item = (&'a str, &'a str)>,
    children: impl Iterator<Item = &'a mut Node<M>>,
) -> web_sys::Element {
    let dom_element = match document().create_element(name) {
        Ok(dom_element) => dom_element,
        Err(_) => todo!(),
    };

    set_attributes(&dom_element, attrs);
    add_children(&dom_element, children);

    return dom_element;
}

fn set_attributes<'a>(
    dom_element: &web_sys::Element,
    attrs: impl Iterator<Item = (&'a str, &'a str)>,
) {
    for (name, val) in attrs {
        match dom_element.set_attribute(name, val) {
            Ok(()) => (),
            Err(_) => todo!(),
        }
    }
}

fn add_children<'a, M: 'static>(
    dom_element: &web_sys::Element,
    children: impl Iterator<Item = &'a mut Node<M>>,
) {
    for child in children {
        node(child);

        // unwrapping is safe because 'draw::element()' must have provided
        match dom_element.append_child(&child.dom_node().unwrap()) {
            Ok(_appended_child) => (),
            Err(_) => todo!(),
        }
    }
}

// #[cfg(test)]
// pub mod tests {
//     use crate::{draw, nodes::Element};
//     use wasm_bindgen_test::*;
//     wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

//     #[wasm_bindgen_test]
//     fn draw_tree() {
//         let body = crate::utils::document().body().unwrap();
//         let element = Element::<()>::new("div")
//             .attr("height", "1000px")
//             .child(Element::new("h1").child("Testje"))
//             .child("Klein dom boompje")
//             .draw()
//             .attach(&body);

//         console_log!("{:#?}", element);
//     }
// }
