use std::thread;

impl rustflow {
    pub fn run() {
        // query device resources
        // create the appropriate workers
        // spawn them
        const CPU_WORKERS: u32 = 4;
        // for i in 0..CPU_WORKERS {}

        let mut e: Executor::new();
    }

    pub fn emplace<T>(&mut self, f: T)
    where
        T: FnMut,
    {
    }
}
struct rustflow {
    // e: Executor,
    g: TaskGraph,
}
pub mod rustflow {}

// uses pipelines for parallelism?
// compatibility with rayon?

/*
This is super weeird now, beause I need to figure out
- do i want to implement my own threadpool and risk perf loss?
- the main objectiveis to integrate heterogenous computing, maybe extend across multiple nodes?
--> heuristics for offloading ot new things?
- spin up a new thread for every task? or pin a thread to a worker which deals with tasks?
 - good branch idea
*/
