#![allow(non_snake_case)]

#[derive(Debug)]
struct Fragment(pub i32);

trait IntoFragments {
    fn push_fragments(self, buffer: &mut Vec<Fragment>);
}

impl IntoFragments for Fragment {
    fn push_fragments(self, buffer: &mut Vec<Fragment>) {
        buffer.push(self)
    }
}

impl<A: IntoFragments, B: IntoFragments> IntoFragments for (A, B) {
    fn push_fragments(self, buffer: &mut Vec<Fragment>) {
        self.0.push_fragments(buffer);
        self.1.push_fragments(buffer);
    }
}

// ----------------- //

fn App() -> impl IntoFragments {
    ((Fragment(0), Fragment(1)), Fragment(2))
}

fn main() {
    let mut buffer: Vec<Fragment> = vec![];
    let app = App();

    app.push_fragments(&mut buffer);

    // Analogous to sending the primitives to be drawn!
    dbg!(buffer);
}
