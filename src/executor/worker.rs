use crate::{task::Task, task_queue::TaskQueue};

struct Worker {
    stop: bool,
    // t: task::Task,
    queue: TaskQueue,
    // thread runner : can be CPU or GPU or other
}

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

fn execute_task(w: &mut Worker, t: &Task) {
    // invoke t
}

fn wait_yield() {}
