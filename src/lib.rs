mod executor;
// mod algorithm;
mod pipeline;
mod task;
mod task_graph;
mod task_queue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub trait Context {}