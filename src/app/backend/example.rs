//! Example of how to create a [`super::Backend`].

use {
    super::Backend,
    crate::fp::Reifiable,
    core::{fmt::Debug, marker::PhantomData},
};

/// The Backend will have an implementation of [`Backend::run`],
/// so it needs an associated [`Backend::Tree`] type, which
/// can be a single concrete type or a generic parameter with arbitrary bounds.
pub struct ExampleBackend<TreeT> {
    _marker: PhantomData<TreeT>,
}

pub struct ExampleBackendContext {
    log_name: &'static str,
}

pub trait ExampleBackendRequirements: Debug {}

impl<'re, TreeT> Backend for ExampleBackend<TreeT>
where
    TreeT: Reifiable<'re, ExampleBackendContext>,
    TreeT::Output: ExampleBackendRequirements,
{
    type Tree = TreeT;

    fn run(node_tree: Self::Tree) {
        let context = ExampleBackendContext { log_name: "LOG" };
        let node_tree = node_tree.reify(context);
        println!("{:?}", node_tree);
    }
}

#[test]
fn example_backend() {
    use crate::app::UIComposer;

    let node_tree = Node { data: "My data" };
    UIComposer::run_custom::<ExampleBackend<_>>(node_tree);
}

struct Node<'a> {
    data: &'a str,
}

#[derive(Debug)]
#[allow(unused)]
struct NodeRe<'re> {
    data: &'re str,
    log_name: &'re str,
}

impl<'a: 're, 're> Reifiable<'re, ExampleBackendContext> for Node<'a> {
    type Output = NodeRe<'re>;

    fn reify(self, cx: ExampleBackendContext) -> Self::Output {
        NodeRe {
            log_name: cx.log_name,
            data: self.data,
        }
    }
}

impl<'re> ExampleBackendRequirements for NodeRe<'re> {}
