// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

trait Executable {
    fn execute();
}

struct StaticFn {
    // func: Fn(),
}
struct Module {
    
}
struct Subflow {
    // subflows should be executed as their own task flow??
}

enum TaskType {
    StaticFn(StaticFn),
    Subflow(Subflow),
    Module(Module),
}
pub struct Task {
    callable: TaskType,
}
