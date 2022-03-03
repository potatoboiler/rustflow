// use https://static.rust-lang.org/doc/master/std/sync/mpsc/index.html instead?
use crate::task::Task;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

pub struct TaskQueue<'a> {
    queue: VecDeque<Task<'a>>,
    _affinity: u32,
    // process affinity?
}

impl<'a> Deref for TaskQueue<'a> {
    type Target = VecDeque<Task<'a>>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl<'a> DerefMut for TaskQueue<'a> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}
