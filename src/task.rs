// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

use std::ops::Deref;

trait Executable {
    fn execute(&mut self);
}

impl<'a> StaticFn<'a> {
    fn new<T>(func: T) -> StaticFn<'a>
    where
        T: FnMut() + 'a,
    {
        StaticFn {
            function: Box::<T>::new(func),
        }
    }
}
impl<'a> Executable for StaticFn<'a> {
    fn execute(&mut self) {
        (self.function)();
    }
}
pub struct StaticFn<'a> {
    function: Box<dyn FnMut() + 'a>,
}
impl Executable for Module {
    fn execute(&mut self) {}
}
pub struct Module {}
impl Executable for Subflow {
    fn execute(&mut self) {}
}
pub struct Subflow {
    // subflows should be executed as their own task flow??
}

/* Taken from the original taskflow source
TaskType::PLACEHOLDER     ->  "placeholder"
TaskType::CUDAFLOW        ->  "cudaflow"
TaskType::SYCLFLOW        ->  "syclflow"
TaskType::STATIC          ->  "static"
TaskType::DYNAMIC         ->  "subflow"
TaskType::CONDITION       ->  "condition"
TaskType::MULTI_CONDITION ->  "multi_condition"
TaskType::MODULE          ->  "module"
TaskType::ASYNC           ->  "async"
TaskType::RUNTIME         ->  "runtime"
 */
pub enum TaskType<'a> {
    StaticFn(StaticFn<'a>),
    Subflow(Subflow),
    Module(Module),
}
impl<'a> Deref for Task<'a> {
    type Target = TaskType<'a>;
    fn deref(&self) -> &Self::Target {
        &self.callable
    }
}
pub struct Task<'a> {
    callable: TaskType<'a>,
}
