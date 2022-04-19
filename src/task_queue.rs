// use https://static.rust-lang.org/doc/master/std/sync/mpsc/index.html instead?
use crate::task::Task;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

pub(crate) struct TaskQueue {
    queue: VecDeque<Task>,
    _affinity: u32,
    // process affinity?
}

impl Deref for TaskQueue {
    type Target = VecDeque<Task>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl DerefMut for TaskQueue {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}

impl TaskQueue {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            queue: Default::default(),
            _affinity: id,
        }
    }
}
