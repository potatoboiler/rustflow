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

impl<'a> Node<'a> {
    pub fn precede(&mut self, n: Node<'a>) {
        self.successors.push(Arc::<Node>::new(n));
        // n.dependents.push(self);
    }

    pub(super) fn execute(&mut self) {
        self.executable.execute()
    }
}

// pub fn precede<'a>(predecessor: &mut Node<'a>, successor: Node<'a>) {
// predecessor.successors.push(Arc::<Node>::new(successor));
// }
pub struct Node<'a> {
    executable: task::Task<'a>,
    successors: Vec<Arc<Node<'a>>>,
    dependents: Vec<Arc<Node<'a>>>,
}

#[test]
fn node_precede_test() {
    let mut a = Node {
        executable: task::Task::from_fn(|| {}),
        successors: Vec::default(),
        dependents: Vec::default(),
    };
    let b = Node {
        executable: task::Task::from_fn(|| {}),
        successors: Vec::default(),
        dependents: Vec::default(),
    };
    let c = Node {
        executable: task::Task::from_fn(|| {}),
        successors: Vec::default(),
        dependents: Vec::default(),
    };
    let d = Node {
        executable: task::Task::from_fn(|| {}),
        successors: Vec::default(),
        dependents: Vec::default(),
    };
    let e = Node {
        executable: task::Task::from_fn(|| {}),
        successors: Vec::default(),
        dependents: Vec::default(),
    };
    precede!(a, b, c, d, e);
    assert_eq!(a.successors.len(), 4);
    for node in a.successors {
        // node.get_mut().execute();
    }
}
