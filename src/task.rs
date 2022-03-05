// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

use std::ops::{Deref, DerefMut};

trait Executable {
    fn execute(&mut self);
}

// union FnType<'a> {
// Fn: Box<dyn Fn() + 'a>,
// FnOnce: Box<dyn FnOnce() + 'a>,
// FnMut: Box<dyn FnMut() + 'a>,
// }
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
// eventually need to refactor to use trait unions (or implement my own) based on below
// may need to also make this generic.
// https://github.com/mahkoh/trait-union/blob/master/proc/src/lib.rs
pub struct StaticFn<'a> {
    function: Box<dyn FnMut() + 'a>,
}
#[test]
fn test1() {
    let mut x: i32 = 0;
    {
        let mut foo = StaticFn::new(|| x += 1);
        foo.execute();
    }
    assert_eq!(x, 1);
}
#[test]
fn non_mut() {
    let mut foo = StaticFn::new(|| println!("test"));
    foo.execute();
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
    fn new<T>(f: TaskType<'a>) -> Task<'a> {
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
                    callable: TaskType::StaticFn(StaticFn::new(|| {
                        println!("how did you end up here?")
                    })),
                }
            }
        }
    }
    pub(crate) fn from_fn<T>(f: T) -> Task<'a>
    where
        T: FnMut() + 'a,
    {
        Task {
            callable: TaskType::StaticFn(StaticFn::new(f)),
        }
    }

    pub(super) fn execute(&mut self) {
        // (self.callable).execute();
        match &mut self.callable {
            // why do i need to use &
            // refactor to impl execute on TaskType variant?? so i can get rid of this match
            TaskType::StaticFn(t) => t.execute(),
            TaskType::Module(m) => m.execute(),
            TaskType::Subflow(s) => s.execute(),
            _ => println!("asdf"),
        };
        // &mut self.deref_mut().execute();
    }
}
pub struct Task<'a> {
    callable: TaskType<'a>,
}
