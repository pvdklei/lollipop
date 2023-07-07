use std::{future::Future, pin::Pin};

pub enum Action<M> {
    Diff,
    Nothing,
    Async(Pin<Box<dyn Future<Output = M>>>),
}

impl<F, M> From<F> for Action<M>
where
    F: Future<Output = M> + 'static,
{
    fn from(f: F) -> Self {
        Action::Async(Box::pin(f))
    }
}
