use crate::task;

struct worker {
    stop: bool,
    t: task::task,
    task_queue: task_queue,
    // thread runner : can be CPU or GPU or other
}

fn ABP_worker_loop(w: worker) {
    let t: Option<task::task> = None;
    while !w.stop {
        match t {
            Some(p) => println!("test!"),
            None => wait_yield(),
        }
    }
}

fn execute_task(w: worker, t: task) {
    // invoke t 
}

fn wait_yield() {}
