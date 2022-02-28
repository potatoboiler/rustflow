// implements the callable task object... not the hierarchies
// Multiple task types:
// - static
// - dynamic
// - composable tasks ???

struct StaticFn {}
struct Module {}
struct Subflow {}

enum TaskType {
    StaticFn(StaticFn),
    Subflow(Subflow),
    Module(Module),
}
pub struct Task {
    callable: TaskType,
}
