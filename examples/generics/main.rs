trait InteractionNode {
    fn handle_event(&self, e: &());
}

struct DebugInteractionNode(String);
impl InteractionNode for DebugInteractionNode {
    fn handle_event(&self, _: &()) {
        dbg!(&self.0);
    }
}

impl<A, B> InteractionNode for (A, B)
where
    A: InteractionNode,
    B: InteractionNode,
{
    fn handle_event(&self, e: &()) {
        self.0.handle_event(e);
        self.1.handle_event(e);
    }
}

impl<T: InteractionNode> InteractionNode for Vec<T> {
    fn handle_event(&self, e: &()) {
        todo!()
    }
}

fn main() {
    let a = App();

    a.handle_event(&());
}

#[allow(non_snake_case)]
fn App() -> impl InteractionNode {
    vec![
        DebugInteractionNode("I was clicked".to_string()),
        DebugInteractionNode("I was not".to_string()),
        DebugInteractionNode("I am Bob".to_string()),
    ]
}
