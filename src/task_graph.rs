// mod task {}
// be able to implement tasks, subflows, and modules as nodes?
mod node;
use node::Node;

pub struct TaskGraph {
    nodes: Vec<Node>,
}
