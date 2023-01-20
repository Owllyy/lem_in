use crate::{Graph, Id};
use super::{AccessRecord, Branch};
pub struct Backtrace<'a> {
    current: Branch,
    graph: &'a Graph,
    accesses: &'a [AccessRecord],
}

impl<'a> Backtrace<'a> {
    pub fn new(graph: &'a Graph, accesses: &'a [AccessRecord], current: Branch) -> Self {
        Self {
            current,
            graph,
            accesses,
        }
    }
}

impl<'a> Iterator for Backtrace<'a> {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.node == self.graph.start() {
            return None;
        }

        let result = self.current.node;
        self.current = Branch{ ..self.accesses[usize::from(result)][&self.current.id] };
        Some(result)
    }
}
