pub mod element;
pub mod node;
pub mod text;
pub mod widget;

 

use std::{any::Any, collections::HashMap, mem::MaybeUninit, sync::Once};

pub fn update_widget<M>(name: &str, msg: M)
where
    M: 'static,
{
    let update = updates()
        .get(name)
        .unwrap()
        .downcast_ref::<Box<dyn Fn(&str, M)>>()
        .unwrap();
    update(name, msg);
}

fn updates() -> &'static mut HashMap<String, Box<dyn Any>> {
    static mut ONCE: Once = Once::new();
    static mut UPDATES: MaybeUninit<HashMap<String, Box<dyn Any>>> = MaybeUninit::uninit();

    unsafe {
        ONCE.call_once(|| {
            UPDATES.write(HashMap::new());
        });

        UPDATES.assume_init_mut()
    }
}

fn properties() -> &'static mut HashMap<String, Box<dyn Any>> {
    static mut ONCE: Once = Once::new();
    static mut PROPS: MaybeUninit<HashMap<String, Box<dyn Any>>> = MaybeUninit::uninit();

    unsafe {
        ONCE.call_once(|| {
            PROPS.write(HashMap::new());
        });

        PROPS.assume_init_mut()
    }
}

fn states() -> &'static mut HashMap<String, Box<dyn Any>> {
    static mut ONCE: Once = Once::new();
    static mut STATES: MaybeUninit<HashMap<String, Box<dyn Any>>> = MaybeUninit::uninit();

    unsafe {
        ONCE.call_once(|| {
            STATES.write(HashMap::new());
        });

        STATES.assume_init_mut()
    }
}

fn nodes() -> &'static mut HashMap<String, Box<dyn Any>> {
    static mut ONCE: Once = Once::new();
    static mut NODES: MaybeUninit<HashMap<String, Box<dyn Any>>> = MaybeUninit::uninit();

    unsafe {
        ONCE.call_once(|| {
            NODES.write(HashMap::new());
        });

        NODES.assume_init_mut()
    }
}
