// mod task {}
// be able to implement tasks, subflows, and modules as nodes?
mod node;
use node::Node;

pub(crate) struct TaskGraph {
    nodes: Vec<Node>,
}

impl TaskGraph {
    pub(crate) fn new() -> TaskGraph {
        TaskGraph {
            nodes: Vec::<Node>::default(),
        }
    }
}
