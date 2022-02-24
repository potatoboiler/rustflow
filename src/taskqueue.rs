use std::collections::VecDeque;
use task::task;

// pub mod taskqueue;
mod task;

pub struct taskqueue {
    queue: VecDeque<task>,
    // process affinity?
}

