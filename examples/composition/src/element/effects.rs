use crate::runner::ElementEffectHandler;

use super::ElementEffect;

pub struct LogEffect(pub String);

impl ElementEffect for LogEffect {
    fn apply(&self, _: &mut ElementEffectHandler) {
        println!("Element Log: {}", self.0)
    }
}
