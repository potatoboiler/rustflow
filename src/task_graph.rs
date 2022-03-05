// mod task {}
// be able to implement tasks, subflows, and modules as nodes?
mod node;
use node::Node;
use std::sync::Arc;

pub(crate) struct TaskGraph<'a> {
    nodes: Vec<Arc<Node<'a>>>,
}

impl<'a> TaskGraph<'a> {
    pub(crate) fn new() -> TaskGraph<'a> {
        TaskGraph {
            nodes: Vec::<Arc<Node>>::default(),
        }
    }
}
