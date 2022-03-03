// use https://static.rust-lang.org/doc/master/std/sync/mpsc/index.html instead?
use crate::task::Task;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

pub struct TaskQueue<'a, T> {
    queue: VecDeque<Task<'a, T>>,
    _affinity: u32,
    // process affinity?
}

impl<'a, T> Deref for TaskQueue<'a, T> {
    type Target = VecDeque<Task<'a, T>>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl<'a, T> DerefMut for TaskQueue<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}
