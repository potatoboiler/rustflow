// use https://static.rust-lang.org/doc/master/std/sync/mpsc/index.html instead?
use crate::task::Task;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

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
