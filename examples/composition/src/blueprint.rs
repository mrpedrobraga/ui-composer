use crate::element::Element;

pub trait Blueprint {
    type Output: Element;

    fn make(&mut self) -> Self::Output;
}
