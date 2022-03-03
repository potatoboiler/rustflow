use std::thread::Thread;

use crate::{task::Task, task_queue::TaskQueue};

use super::{Atomic, Domain, Scheduler};

// consider refactoring as https://stackoverflow.com/questions/59426358/how-to-make-a-struct-containing-an-enum-generic-over-the-enum-variant
enum ThreadType {
    // GPU corresponds to nvidia SM?
    // use wgpu or rust-cuda?
    GPU,
    CPU(Thread),
}

// consider refactoring ref to parent as https://www.reddit.com/r/rust/comments/cnjhup/idiomatic_way_to_reference_parent_struct/ewb5r1y/
pub(super) struct Worker<'a> {
    _stop: bool,
    // t: task::Task,
    queue: TaskQueue<'a>,
    dom: Domain,
    thread: ThreadType,
    // curr_task: mut Option<Task>,
    // thread runner : can be CPU or GPU or other
}

// cited algorithm in paper
/*
fn abp_worker_loop(w: &mut Worker) {
    let mut t: Option<Task> = None;
    while !w.stop {
        match t {
            Some(ref a) => execute_task(w, a),
            None => wait_yield(),
        }
        t = w.queue.pop_back();
    }
}
*/

// actually does stuff
impl<'a> Worker<'a> {
    // move all functions into here
    pub(super) fn new(s: &mut Scheduler) {}
}
fn actual_worker_loop(w: &mut Worker, s: &mut Scheduler) {
    // let t: &Option<Task> = &None;
    loop {
        exploit_task(w, s);
        // if !wait_for_task(w, ) {
        // break;
        // }
    }
}
use super::WorkerAction::*;
// behavior will start to differ because we are no longer passing a NIL task into the function, we are popping it directly
fn exploit_task(w: &mut Worker, s: &mut Scheduler) {
    let t: &Option<Task> = &w.queue.pop_back();
    if !t.is_none() {
        if s.atom_inc(Actives, &w.dom) == 1 && s.atom_load(Actives, &w.dom) == 0 {
            // notify one
        }
        loop {
            execute_task(w, t);
            if t.is_none() {
                break;
            };
        }
    }
    s.atom_dec(Actives, &w.dom);
}
fn wait_for_task(w: &mut Worker, t: &Option<Task>) -> bool {
    // FIXME
    false
}

fn execute_task(w: &mut Worker, t: &Option<Task>) {
    match w {
        Worker { dom: CPU, .. } => println!("place"),
        Worker { dom: GPU, .. } => println!("place"),
    }
}

fn wait_yield() {}
