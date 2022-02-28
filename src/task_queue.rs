use crate::task::Task;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
// use task::task;

// pub mod taskqueue;

pub struct TaskQueue {
    queue: VecDeque<Task>,
    _affinity: u32,
    // process affinity?
}

impl Deref for TaskQueue {
    type Target = VecDeque<Task>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl DerefMut for TaskQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}
