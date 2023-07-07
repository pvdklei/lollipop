pub use crate::{
    action::Action,
    events::ListenerKind,
    nodes::{element::Element as Html, widget::Widget},
    utils::log,
};

pub fn run<M: 'static>(w: Widget) {
    let body = crate::utils::document().body().unwrap();
    w.node::<M>().attach_borrow(&body);
}

macro_rules! implement_tags {
    ( $( $tag:ident ),* ) => {
        $(
            pub fn $tag<M>() -> Html<M> {
                Html::new(stringify!($tag))
            }
        )*
    };
}

implement_tags!(div, p, h1, h2, h3, h4, h5, h6, h7, img, a, button, input, form, object);

// macro_rules! implement_operator {
//     ($trait:ident, $fname:ident) => {
//         impl<M> $trait<Self> for Html<M> {
//             type Output = Self;
//             fn $fname(self, rhs: Self) -> Self::Output {
//                 self.child(rhs)
//             }
//         }

//         impl<M> $trait<String> for Html<M> {
//             type Output = Self;
//             fn $fname(self, rhs: String) -> Self::Output {
//                 self.child(rhs)
//             }
//         }

//         impl<M> $trait<&'static str> for Html<M> {
//             type Output = Self;
//             fn $fname(self, rhs: &'static str) -> Self::Output {
//                 self.child(rhs)
//             }
//         }

//         impl<M> $trait<Widget> for Html<M> {
//             type Output = Self;
//             fn $fname(self, rhs: Widget) -> Self::Output {
//                 self.child(rhs)
//             }
//         }

//         impl<M> $trait<(&'static str, ListenerKind<M>)> for Html<M> {
//             type Output = Self;
//             fn $fname(self, rhs: (&'static str, ListenerKind<M>)) -> Self::Output {
//                 self.on_listener_kind(rhs.0, rhs.1)
//             }
//         }

//         impl<M> $trait<(&'static str, String)> for Html<M> {
//             type Output = Self;
//             fn $fname(self, rhs: (&'static str, String)) -> Self::Output {
//                 self.attr(rhs.0, rhs.1)
//             }
//         }
//     };
// }

// implement_operator!(Add, add);
// implement_operator!(Mul, mul);
// implement_operator!(BitAnd, bitand);
// implement_operator!(BitOr, bitor);



// macro_rules! implement_listeners {
//         ( $( $listener:ident ),* ) => {
//             $(
//                 pub fn $listener<M>(callback: impl Into<ListenerKind<M>>) -> (&'static str, ListenerKind<M>) {
//                     (stringify!($listener), callback.into())
//                 }
//             )*
//         };
//     }

// implement_listeners!(click, mouseenter, mouseleave, change);

// macro_rules! implement_attributes {
//     ( $( $attr:ident ),* ) => {
//         $(
//             pub fn $attr(value: impl ToString) -> (&'static str, String) {
//                 (stringify!($attr), value.to_string())
//             }
//         )*
//     };
// }

// implement_attributes!(width, height, src, type_);
