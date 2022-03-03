// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

use std::ops::Deref;

trait Executable {
    fn execute(&self);
}

impl<'a, T> StaticFn<'a, T> {
    fn new(func: T) -> StaticFn<'a, T>
    where
        T: Fn() + 'a,
    {
        StaticFn {
            function: Box::<T>::new(func),
        }
    }
}
impl<'a, T> Executable for StaticFn<'a, T> {
    fn execute(&self) {
        (self.function)()
    }
}
// https://github.com/rhaiscript/rhai/blob/main/src/func/func.rs
// RHAI has some nice generic fn pointers?
// function: Box<dyn Fn() -> ()>,
pub struct StaticFn<'a, T>
where
    T: Fn() + 'a,
{
    function: Box<T>,
}
impl Executable for Module {
    fn execute(&self) {}
}
pub struct Module {}
impl Executable for Subflow {
    fn execute(&self) {}
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
pub enum TaskType<'a, T> {
    StaticFn(StaticFn<'a, T>),
    Subflow(Subflow),
    Module(Module),
}
impl<'a, T> Deref for Task<'a, T> {
    type Target = TaskType<'a, T>;
    fn deref(&self) -> &Self::Target {
        &self.callable
    }
}
pub struct Task<'a, T> {
    callable: TaskType<'a, T>,
}
