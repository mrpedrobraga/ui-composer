use std::ops::Range;

#[derive(Debug, Clone)]
struct Primitive(String);

struct GenericReactiveNode<F: Fn(i32) -> Vec<Primitive>> {
    dirty: bool,
    current_value: i32,
    range: Range<usize>,
    replacer: F,
}

trait ReactiveNode {
    fn set(&mut self, new_value: i32);
    fn is_dirty(&self) -> bool;
    fn apply(&mut self, buffer: &mut Vec<Primitive>);
}

impl<F: Fn(i32) -> Vec<Primitive>> ReactiveNode for GenericReactiveNode<F> {
    fn set(&mut self, new_value: i32) {
        self.current_value = new_value;
        self.dirty = true;
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn apply(&mut self, buffer: &mut Vec<Primitive>) {
        buffer.splice(self.range.clone(), {
            let rep = &self.replacer;
            rep(self.current_value).iter().cloned()
        });
        self.dirty = false;
    }
}

struct World {
    primitives: Vec<Primitive>,
    reactive_nodes: Vec<Box<dyn ReactiveNode>>,
}

impl World {
    fn render(&self) {
        println!(
            "{}",
            self.primitives.iter().fold(String::new(), |mut s, p| {
                s.push_str(p.0.as_str());
                s
            })
        )
    }

    fn update(&mut self) {
        for reactive_node in self.reactive_nodes.iter_mut() {
            if reactive_node.is_dirty() {
                reactive_node.apply(&mut self.primitives)
            }
        }
    }
}

fn main() {
    let mut world = World {
        primitives: vec![Primitive("A(0)".to_string()), Primitive("B".to_string())],
        reactive_nodes: vec![Box::new(GenericReactiveNode {
            dirty: false,
            current_value: 0,
            range: 0..1,
            replacer: |num| vec![Primitive(format!("A({})", num))],
        })],
    };

    world.render();
    world.reactive_nodes[0].set(3);
    world.update();
    world.render();
}
