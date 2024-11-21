use crate::graph::DummyGraph;

pub struct GraphAdapter<'a> {
    pub graph: &'a mut DummyGraph,
}

impl<'a> GraphAdapter<'a> {
    pub fn new(graph: &'a mut DummyGraph) -> Self {
        Self { graph }
    }
}
