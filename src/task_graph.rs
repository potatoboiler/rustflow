// mod task {}
// be able to implement tasks, subflows, and modules as nodes?
mod node;
use node::Node;
use std::sync::Arc;

pub(crate) struct TaskGraph {
    nodes: Vec<Arc<Node>>,
}

impl TaskGraph {
    pub(crate) fn new() -> TaskGraph {
        TaskGraph {
            nodes: Vec::<Arc<Node>>::default(),
        }
    }
}
