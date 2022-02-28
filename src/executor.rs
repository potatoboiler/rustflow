use std::{
    collections::HashMap, hash::Hash, sync::atomic::AtomicU32, sync::atomic::Ordering::Relaxed,
};

use crate::task_graph::TaskGraph;
// executor can take workers and assign them to nodes --> invoke(worker, node)
// find topological sort? (is there any way to have this weighted?)

// runtime is a separate class?
// each thread in the pool has a work queue that can get stolen from
mod worker;
use worker::Worker;

#[derive(Eq, Hash)]
enum Domain {
    GPU,
    CPU,
}
impl PartialEq for Domain {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
enum ActThi {
    Actives,
    Thieves,
}

trait Atomic {
    fn AtomInc(&mut self, a: ActThi, d: &Domain) -> u32;
    fn AtomDec(&mut self, a: ActThi, d: &Domain) -> u32;
}
impl Atomic for Scheduler {
    fn AtomInc(&mut self, a: ActThi, d: &Domain) -> u32 {
        let map: &mut HashMap<Domain, AtomicU32> = match a {
            ActThi::Actives => &mut self.actives,
            ActThi::Thieves => &mut self.thieves,
        };
        map.get_mut(d).unwrap().fetch_add(1, Relaxed) + 1
    }
    fn AtomDec(&mut self, a: ActThi, d: &Domain) -> u32 {
        let map: &mut HashMap<Domain, AtomicU32> = match a {
            ActThi::Actives => &mut self.actives,
            ActThi::Thieves => &mut self.thieves,
        };
        map.get_mut(&d).unwrap().fetch_sub(1, Relaxed) - 1
    }
}
struct Scheduler {
    workers: Vec<Worker>,
    actives: HashMap<Domain, AtomicU32>,
    thieves: HashMap<Domain, AtomicU32>,
    // activesLock:
}
struct Executor {
    graph: TaskGraph,
    scheduler: Scheduler,
}
