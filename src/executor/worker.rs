use crate::{task::Task, task_queue::TaskQueue};

use super::Domain;
pub struct Worker {
    _stop: bool,
    // t: task::Task,
    queue: TaskQueue,
    dom: Domain,
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
fn actual_worker_loop(w: &mut Worker) {
    let mut t: Option<Task> = None;
    loop {
        exploit_task(w, &t);
        if !wait_for_task(w, &t) {
            break;
        }
    }
}
fn exploit_task(w: &mut Worker, t: &Option<Task>) {
    if let Task = t {
        if (true) {}
    }
}
fn wait_for_task(w: &mut Worker, t: &Option<Task>) -> bool {
    false
}

fn execute_task(w: &mut Worker, t: &Task) {
    // invoke t
}

fn wait_yield() {}
