// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

use std::ops::Deref;

trait Executable {
    fn execute();
}

impl Executable for StaticFn {
    fn execute() {}
}
pub struct StaticFn {
    // func: Fn(),
// args: Vec<any>,
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

pub enum TaskType {
    StaticFn(StaticFn),
    Subflow(Subflow),
    Module(Module),
}
impl Deref for Task {
    type Target = TaskType;
    fn deref(&self) -> &Self::Target {
        &self.callable
    }
}
pub struct Task {
    callable: TaskType,
}
