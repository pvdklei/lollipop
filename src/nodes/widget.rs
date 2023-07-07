use std::{any::Any, collections::hash_map::Entry};

use crate::{action::Action, diff, draw, events};

use wasm_rs_async_executor::single_threaded::{spawn};

use super::{node::Node, nodes, properties, states, update_widget, updates};

#[derive(Clone, Debug)]
pub struct Widget {
    pub name: String,
    pub dom_node: web_sys::Node,
}

impl Widget {
    pub fn new<P, S, M, N>(
        name: &str,
        props: P,
        state: fn() -> S,
        update: fn(&mut S, &P, M) -> Action<M>,
        view: fn(&S, &P) -> N
    ) -> Widget
    where
        P: PartialEq + 'static,
        S: Any,
        M: 'static,
        N: Into<Node<M>> + 'static,
    {
        let state = states()
            .entry(name.to_string())
            .or_insert_with(|| Box::new(state()))
            .downcast_ref::<S>()
            .unwrap();

        match nodes().entry(name.to_string()) {
            // diff nodes if props are different
            Entry::Occupied(mut on) => {
                match properties().entry(name.to_string()) {
                    Entry::Occupied(mut op) => {
                        if op.get().downcast_ref::<P>().unwrap() == &props {
                            // nothing needs to update
                        } else {
                            let mut new_node = view(state, &props).into();
                            let old_node = on.get_mut().downcast_mut::<Node<M>>().unwrap();
                            op.insert(Box::new(props));
                            diff::diff_node(name, old_node, &mut new_node);
                            on.insert(Box::new(new_node));
                        }
                    }
                    Entry::Vacant(vp) => {
                        let mut new_node = view(state, &props).into();
                        let old_node = on.get_mut().downcast_mut::<Node<M>>().unwrap();
                        vp.insert(Box::new(props));
                        diff::diff_node(name, old_node, &mut new_node);
                        on.insert(Box::new(new_node));
                    }
                };
            }

            // draw node and connect children widgets
            Entry::Vacant(vn) => {
                let mut new_node = view(state, &props).into();
                properties().insert(name.to_string(), Box::new(props));
                draw::node(&mut new_node);
                events::listen(name, &mut new_node);
                vn.insert(Box::new(new_node));
            }
        };

        updates().insert(
            name.to_string(),
            Box::new(Box::new(move |name: &str, msg: M| {
                let props = properties().get(name).unwrap().downcast_ref::<P>().unwrap();
                let state = states().get_mut(name).unwrap().downcast_mut::<S>().unwrap();
                let action = update(state, props, msg);

                match action {
                    Action::Diff => {
                        let mut new_node = view(state, props).into();
                        let old_node = nodes().get_mut(name).unwrap().downcast_mut().unwrap();
                        diff::diff_node(name, old_node, &mut new_node);
                        nodes().insert(name.to_string(), Box::new(new_node));
                    }
                    Action::Nothing => {}
                    Action::Async(f) => {
                        let name = name.to_string();
                        spawn(async move {
                            let msg = f.await;
                            update_widget(&name, msg);
                        });
                    }
                }
            }) as Box<dyn Fn(&str, M)>) as Box<dyn Any>,
        );

        let dom_node = nodes()
            .get(name)
            .unwrap()
            .downcast_ref::<Node<M>>()
            .unwrap()
            .dom_node()
            .unwrap()
            .clone();

        Widget {
            name: name.to_string(),
            dom_node,
        }
    }

    pub fn node<M>(self) -> Node<M> {
        Node::Widget(self)
    }
}
