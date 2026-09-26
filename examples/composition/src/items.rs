use crate::element::{effects::LogEffect, Element};

pub struct Text(pub String);

pub struct Number(pub i32);

impl Element for Text {
    type Effect = LogEffect;

    fn effect(&self) -> Self::Effect {
        LogEffect(self.0.clone())
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        std::task::Poll::Ready(None)
    }
}

#[macro_export]
macro_rules! list {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, list!($($rest)*))
    };
}
