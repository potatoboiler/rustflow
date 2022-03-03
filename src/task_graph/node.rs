// use std::option;

use crate::task;
pub struct Node<'a> {
    executable: task::Task<'a>,
    // successors: Vec<&Node>,
}
