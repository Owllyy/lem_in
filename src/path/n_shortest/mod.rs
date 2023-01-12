mod backtrace;
use backtrace::Backtrace;

use super::Path;
use crate::{BitArray, Graph};
use std::collections::{HashMap, VecDeque};

type PathId = usize;

struct ValidPath {
    path_id: PathId,
    hit_node: BitArray,
    incompats: BitArray,
}

struct PathIdGenerator(PathId);

impl PathIdGenerator {
    fn new() -> Self {
        Self(0)
    }

    fn peek(&self) -> PathId {
        self.0
    }

    fn next(&mut self) -> PathId {
        let result = self.0;
        self.0 += 1;
        result
    }
}

fn find_group(incompats: &BitArray, paths: &[ValidPath], count: usize) -> Option<Vec<PathId>> {
    if count == 0 {
        return Some(Vec::new());
    }
    for (path_index, path) in paths.iter().enumerate() {
        if incompats.get(path_index) {
            continue;
        }

        let result = find_group(&(incompats | &path.incompats), paths, count - 1);
        if let Some(mut group) = result {
            group.push(path.path_id);
            return Some(group);
        }
    }
    None
}

impl Path {
    pub fn n_shortest(graph: &Graph, n: usize) -> Option<Vec<Self>> {
        // TODO: find better way
        if n == 0 {
            return Some(Vec::new());
        }
        let mut active_branches = VecDeque::new();
        let mut accesses = vec![HashMap::new(); graph.nodes().len()];
        let mut valid_paths: Vec<ValidPath> = vec![];

        let mut path_id_generator = PathIdGenerator::new();
        active_branches.push_back((path_id_generator.next(), graph.start()));

        let group = loop {
            let (path_id, id) = active_branches.pop_front()?;
            if id == graph.end() {
                let mut hit_node = BitArray::new(graph.nodes().len());

                let mut incompats = BitArray::new(valid_paths.len());
                for id in Backtrace::new(graph, &accesses, path_id) {
                    hit_node.add(id.0);
                    for (path_index, path) in valid_paths.iter().enumerate() {
                        incompats.add_if(path_index, path.hit_node.get(id.0));
                    }
                }

                if let Some(mut group) = find_group(&incompats, &valid_paths, n - 1) {
                    group.push(path_id);
                    break group;
                }

                valid_paths.push(ValidPath { path_id, hit_node, incompats });
                continue;
            }

            for &link in &graph[id].links {
                // /!\ Wrong !
                // The path_id is of the current path segment (we must likely backtrace)
                // checking for it in previous wont prevent loops
                // Example:
                //
                //     #0     #1     #2
                // (1) -> (2) -> (3) -> (1)
                // 
                // >> Here node (1) it moved back to because it doesn't know path #2 
                //
                // Maybe preventing loops wont be usefull
                // We could try to prevent moving back

                accesses[link.0].entry(path_id_generator.peek()).or_insert_with(|| {
                    active_branches.push_back((path_id_generator.next(), link));
                    (path_id, id)
                });
            }
        };
        Some(group
            .into_iter()
            .map(|path_id| Backtrace::new(graph, &accesses, path_id).collect())
            .collect())
    }
}
