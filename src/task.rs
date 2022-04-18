// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

use std::ops::{Deref, DerefMut};

use crate::Context;

// union FnType {
// Fn: Box<dyn Fn() + >,
// FnOnce: Box<dyn FnOnce() + >,
// FnMut: Box<dyn FnMut() + >,
// }
impl StaticFn {
    fn new(func: impl Fn(&mut dyn Context) + 'static) -> StaticFn
    {
        StaticFn {
            function: Box::new(func),
        }
    }
}
// eventually need to refactor to use trait unions (or implement my own) based on below
// may need to also make this generic.
// https://github.com/mahkoh/trait-union/blob/master/proc/src/lib.rs
pub struct StaticFn {
    function: Box<dyn Fn(&mut dyn Context)>,
}

pub struct Module {}
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
/*
impl DerefMut for Task {
    // type Target = TaskType;
    fn deref_mut(&mut self) -> &mut TaskType {
        &mut self.callable
    }
}
*/
// https://stackoverflow.com/questions/25576748/how-to-compare-enum-without-pattern-matching
impl Task {
    // https://crates.io/crates/enum_dispatch
    fn new(f: TaskType) -> Task {
        match f {
            TaskType::StaticFn(t) => Task {
                callable: TaskType::StaticFn(StaticFn::new(t.function)),
            },
            TaskType::Module(m) => Task {
                callable: TaskType::Module(m),
            },
            TaskType::Subflow(s) => Task {
                callable: TaskType::Subflow(s),
            },
            _ => {
                return Task {
                    callable: TaskType::StaticFn(StaticFn::new(|_| {
                        println!("how did you end up here?")
                    })),
                }
            }
        }
    }
}
pub struct Task {
    callable: TaskType,
}
