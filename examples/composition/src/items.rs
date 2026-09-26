use std::sync::OnceLock;

use crate::element::{effects::LogEffect, Element};

pub struct Text(pub String, OnceLock<TextResources>);

pub struct TextResources {
    content: String,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self(content, OnceLock::new())
    }
}

impl Element for Text {
    type Effect = LogEffect;

    fn initialize(&mut self, resources: &crate::runner::InitializationResources) {
        let _ = self.1.set(TextResources {
            content: resources.secret_key.clone(),
        });
    }

    fn effect(&self) -> Self::Effect {
        LogEffect(format!(
            "{} - ({})",
            self.0,
            self.1.get().map(|re| re.content.as_str()).unwrap()
        ))
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
