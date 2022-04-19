use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

use crate::{task::Task, task_queue::TaskQueue};

use super::{Atomic, Domain, Scheduler};

// consider refactoring as https://stackoverflow.com/questions/59426358/how-to-make-a-struct-containing-an-enum-generic-over-the-enum-variant
enum ThreadType {
    CPU,
}

// consider refactoring ref to parent as https://www.reddit.com/r/rust/comments/cnjhup/idiomatic_way_to_reference_parent_struct/ewb5r1y/
pub(crate) struct Worker {
    _stop: bool,
    // t: task::Task,
    id: u32,
    queue: TaskQueue,
    dom: Domain,
    thread: Option<JoinHandle<()>>,
    curr_task: Option<Arc<Task>>,
    sched: Arc<RwLock<Scheduler>>,
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
impl Worker {
    // move all functions into here

    // TODO
    pub(crate) fn new(id: u32, sched: Arc<RwLock<Scheduler>>) -> Worker {
        Worker {
            _stop: false,
            id: id,
            queue: TaskQueue::new(id),
            dom: Domain::CPU,
            curr_task: None,
            sched: sched.clone(),
            thread: None,
        }
    }

    fn worker_loop(&mut self) {
        self.thread = Some(thread::spawn(|| loop {
            self.exploit_task(self.curr_task.clone());
            if self.wait_for_task() {
                break;
            }
        }));
    }

    fn exploit_task(&self, t: Option<Arc<Task>>) {
        let mut sched = self.sched.write().unwrap();
    }

    fn wait_for_task(&self) -> bool {
        false
    }

    fn execute_task(&self) {
        // executes jobs
        self.curr_task.as_ref();
    }

    fn submit_task(&self) {}

    fn steal_task(&self) {}

    fn explore_task(&self) {}
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
