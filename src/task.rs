// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

use std::ops::Deref;

trait Executable {
    fn execute();
}

impl<'a> StaticFn<'a> {
    fn new(func: dyn Fn()) -> StaticFn<'a> {
        StaticFn {
            function: Box::<dyn Fn()>::new(func),
        }
    }
}
impl<'a> Executable for StaticFn<'a> {
    fn execute() {}
}
// https://github.com/rhaiscript/rhai/blob/main/src/func/func.rs
// RHAI has some nice generic fn pointers?
// function: Box<dyn Fn() -> ()>,
pub struct StaticFn<'a> {
    function: Box<dyn Fn() + 'a>,
}
impl Executable for Module {
    fn execute() {}
}
pub struct Module {}
impl Executable for Subflow {
    fn execute() {}
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
