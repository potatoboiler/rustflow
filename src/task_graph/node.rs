// use std::option;

use std::sync::Arc;

use crate::task;

// refactor as https://doc.rust-lang.org/reference/procedural-macros.html ?
#[macro_export]
macro_rules! precede {
    ($a:expr, $($x:expr),*) => {{
        $($a.precede($x);)*
    }};
}

impl Node {
    pub fn precede(&mut self, n: Node) {
        self.successors.push(Arc::<Node>::new(n));
        // n.dependents.push(self);
    }
}

// pub fn precede(predecessor: &mut Node, successor: Node) {
// predecessor.successors.push(Arc::<Node>::new(successor));
// }
pub struct Node {
    executable: task::Task,
    successors: Vec<Arc<Node>>,
    dependents: Vec<Arc<Node>>,
}