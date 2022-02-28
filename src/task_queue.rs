use std::collections::VecDeque;
use crate::task;
// use task::task;

// pub mod taskqueue;

pub struct TaskQueue {
    queue: VecDeque<task::Task>,
    affinity: u32,
    // process affinity?
}

