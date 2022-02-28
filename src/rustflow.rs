trait Flow {
    pub fn run();
}
impl Flow for rustflow {
    fn run() {}
}
pub mod rustflow {}

// uses pipelines for parallelism?
// compatibility with rayon?
