// use std::option;

use crate::task;
pub struct Node<'a, T> {
    executable: task::Task<'a, T>,
    // successors: Vec<&Node>,
}
