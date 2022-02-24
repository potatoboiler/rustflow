// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

mod task {
    pub enum taskType {
        static_fn,
        subflow,
        module,
    }
}

pub struct task {}
