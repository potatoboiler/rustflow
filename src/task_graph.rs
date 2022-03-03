// mod task {}
// be able to implement tasks, subflows, and modules as nodes?
mod node;
use node::Node;

pub(crate) struct TaskGraph<'a> {
    nodes: Vec<Node<'a>>,
}

impl<'a> TaskGraph<'a> {
    pub(crate) fn new() -> TaskGraph<'a> {
        TaskGraph {
            nodes: Vec::<Node>::default(),
        }
    }
}
