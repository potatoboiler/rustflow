use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    sync::atomic::AtomicU32,
    sync::atomic::Ordering::Relaxed,
};

use crate::{task::Task, task_graph::TaskGraph};
// executor can take workers and assign them to nodes --> invoke(worker, node)
// find topological sort? (is there any way to have this weighted?)

// runtime is a separate class?
// each thread in the pool has a work queue that can get stolen from
mod worker;
use self::worker::Worker;

#[derive(Eq, PartialEq, Hash)]
enum Domain {
    GPU,
    CPU,
}
enum WorkerAction {
    Actives,
    Thieves,
}

impl<'a> Scheduler<'a> {
    #[inline(always)]
    fn get_map_atom(&mut self, a: WorkerAction, d: &Domain) -> &AtomicU32 {
        let map: &mut HashMap<Domain, AtomicU32> = match a {
            WorkerAction::Actives => &mut self.actives,
            WorkerAction::Thieves => &mut self.thieves,
        };
        map.get_mut(d).unwrap()
    }
}
trait Atomic {
    fn atom_inc(&mut self, a: WorkerAction, d: &Domain) -> u32;
    fn atom_dec(&mut self, a: WorkerAction, d: &Domain) -> u32;
    fn atom_load(&mut self, a: WorkerAction, d: &Domain) -> u32;
}
impl<'a> Atomic for Scheduler<'a> {
    fn atom_inc(&mut self, a: WorkerAction, d: &Domain) -> u32 {
        self.get_map_atom(a, d).fetch_add(1, Relaxed) + 1
    }
    fn atom_dec(&mut self, a: WorkerAction, d: &Domain) -> u32 {
        self.get_map_atom(a, d).fetch_sub(1, Relaxed) - 1
    }
    fn atom_load(&mut self, a: WorkerAction, d: &Domain) -> u32 {
        self.get_map_atom(a, d).load(Relaxed)
    }
}
pub struct Scheduler<'a> {
    workers: Vec<Worker<'a>>,
    actives: HashMap<Domain, AtomicU32>,
    thieves: HashMap<Domain, AtomicU32>,
    shared_queues: HashMap<Domain, VecDeque<Task<'a>>>, // may need to refactor if extra traits are needed?
    // activesLock:
    notifiers: HashMap<Domain, Notifier>,
}

struct Notifier {}
impl Default for Notifier {
    fn default() -> Notifier {
        Notifier {}
    }
}
pub struct Executor<'a> {
    // graph: TaskGraph<'a>, // does taskgraph need to be member of taskflow?
    scheduler: Scheduler<'a>,
}

impl<'a> Default for Scheduler<'a> {
    fn default() -> Scheduler<'a> {
        Scheduler {
            // refactor default behavior
            workers: Vec::<Worker>::default(),
            actives: HashMap::<Domain, AtomicU32>::from([(Domain::CPU, AtomicU32::new(0))]),
            thieves: HashMap::<Domain, AtomicU32>::from([(Domain::CPU, AtomicU32::new(0))]),
            shared_queues: HashMap::<Domain, VecDeque<Task>>::from([(
                Domain::CPU,
                VecDeque::<Task>::new(),
            )]),
            notifiers: HashMap::<Domain, Notifier>::from([(Domain::CPU, Notifier::default())]),
            ..Default::default()
        }
    }
}
impl<'a> Scheduler<'a> {
    fn new(num_threads: u32, num_gpus: u32) -> Scheduler<'a> {
        // i guess, we assume that gpus are 1-to-1 with workers
        assert!(num_threads > 0);
        let tmp_workers = Vec::<Worker>::new();
        Scheduler {
            // FIXME
            ..Default::default()
        }
    }
}

impl<'a> Executor<'a> {
    fn new() -> Executor<'a> {
        // let Scheduler { workers: 5, actives, thieves, shared_queues, notifiers }
        Executor {
            // FIXME
            // graph: TaskGraph::new(),
            scheduler: Scheduler::default(),
        }
    }
}
