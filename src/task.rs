// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

mod task {
    pub enum taskType {
        static_fn(task::static_fn),
        subflow(task::subflow),
        module(task::module),
    }
}

pub struct task {}
