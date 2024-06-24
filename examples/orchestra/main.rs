use std::{borrow::Borrow, ops::Range};

#[derive(Debug, Clone)]
struct Primitive(String);

struct ReactiveNode {
    dirty: bool,
    current_value: i32,
    range: Range<usize>,
    replacer: Box<dyn Fn(i32) -> Vec<Primitive>>,
}

impl ReactiveNode {
    fn set(&mut self, new_value: i32) {
        self.current_value = new_value;
        self.dirty = true;
    }
}

struct World {
    primitives: Vec<Primitive>,
    reactive_nodes: Vec<ReactiveNode>,
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
            if reactive_node.dirty {
                self.primitives.splice(reactive_node.range.clone(), {
                    let rep = &*reactive_node.replacer;
                    rep(reactive_node.current_value).iter().cloned()
                });
                reactive_node.dirty = false;
            }
        }
    }
}

fn main() {
    let mut world = World {
        primitives: vec![Primitive("A(0)".to_string()), Primitive("B".to_string())],
        reactive_nodes: vec![ReactiveNode {
            dirty: false,
            current_value: 0,
            range: 0..1,
            replacer: Box::new(|num| vec![Primitive(format!("A({})", num))]),
        }],
    };

    world.render();
    world.reactive_nodes[0].set(3);
    world.update();
    world.render();
}
