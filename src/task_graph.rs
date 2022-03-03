// mod task {}
// be able to implement tasks, subflows, and modules as nodes?
mod node;
use node::Node;

pub(crate) struct TaskGraph<'a, T> {
    nodes: Vec<Node<'a, T>>,
}

impl<'a, T> TaskGraph<'a, T> {
    pub(crate) fn new() -> TaskGraph<'a, T> {
        TaskGraph {
            nodes: Vec::<Node>::default(),
        }
    }
}
