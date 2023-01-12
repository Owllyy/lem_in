use crate::{Graph, Id};
use super::PathId;
use std::collections::HashMap;

type Accesses = [HashMap<PathId, (PathId, Id)>];

pub struct Backtrace<'a> {
    current_id: Id,
    current_path_id: PathId,
    graph: &'a Graph,
    accesses: &'a Accesses,
}

impl<'a> Backtrace<'a> {
    pub fn new(graph: &'a Graph, accesses: &'a Accesses, path_id: PathId) -> Self {
        let mut result = Self {
            current_id: graph.end(),
            graph,
            accesses,
            current_path_id: path_id,
        };
        result.next();
        result
   }
}

impl<'a> Iterator for Backtrace<'a> {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id == self.graph.start() {
            return None;
        }

        let result_id = self.current_id;
        let (id_path, id) = self.accesses[result_id.0][&self.current_path_id];

        self.current_id = id;
        self.current_path_id = id_path;

        Some(result_id)
    }
}
