// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

use std::ops::{Deref, DerefMut};

use crate::Context;

// union FnType<'a> {
// Fn: Box<dyn Fn() + 'a>,
// FnOnce: Box<dyn FnOnce() + 'a>,
// FnMut: Box<dyn FnMut() + 'a>,
// }
impl<'a> StaticFn<'a> {
    fn new(func: impl Fn(&mut dyn Context) + 'a) -> StaticFn<'a>
    // where
        // T: FnMut() + 'a,
    {
        StaticFn {
            function: Box::new(func),
        }
    }
}
// eventually need to refactor to use trait unions (or implement my own) based on below
// may need to also make this generic.
// https://github.com/mahkoh/trait-union/blob/master/proc/src/lib.rs
pub struct StaticFn<'a> {
    function: Box<dyn Fn(&mut dyn Context) + 'a>,
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
/*
impl<'a> DerefMut for Task<'a> {
    // type Target = TaskType<'a>;
    fn deref_mut(&mut self) -> &mut TaskType<'a> {
        &mut self.callable
    }
}
*/
// https://stackoverflow.com/questions/25576748/how-to-compare-enum-without-pattern-matching
impl<'a> Task<'a> {
    // https://crates.io/crates/enum_dispatch
    fn new(f: TaskType<'a>) -> Task<'a> {
        match f {
            TaskType::StaticFn(t) => Task::from_fn(t.function),
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
    // this might not be necessary
    pub(crate) fn from_fn(f: impl Fn(&mut dyn Context) + 'a) -> Task<'a>
    {
        Task {
            callable: TaskType::StaticFn(StaticFn::new(f)),
        }
    }
}
pub struct Task<'a> {
    callable: TaskType<'a>,
}
