use std::collections::VecDeque;
use task::task;

// pub mod taskqueue;
mod task;

pub struct task_queue {
    queue: VecDeque<task>,
    affinity: u32,
    // process affinity?
}

