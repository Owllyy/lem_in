use std::{collections::VecDeque};

use crate::{Id, Graph};

#[derive(Debug, Clone)]
pub struct Path(Vec<Id>);

pub fn shortest_path(graph: &Graph) -> Option<Path> {
    let mut accesses = vec![None; graph.nodes().len()];
    let mut active_nodes = VecDeque::new();

    let mut id = graph.start();
    accesses[id.0] = Some(id);
    loop {
        if id == graph.end() {
            break;
        }
        for link in &graph[id].links {
            let access = &mut accesses[link.0];
            if access.is_none() {
                *access = Some(id);
                active_nodes.push_back(link);
            }
        }
        id = *active_nodes.pop_front()?;
    }

    let mut path = vec![id];
    while id != graph.start() {
        id = accesses[id.0].unwrap();
        path.push(id);
    }
    path.reverse();
    Some(Path(path))
}

fn trace_back(graph: &Graph, accesses: &[Option<Id>]) -> Path {
    let mut id = graph.end();
    let mut path = vec![id];
    while id != graph.start() {
        id = accesses[id.0].unwrap();
        path.push(id);
    }
    path.reverse();
    Path(path)
}

pub fn shortest_n_paths(graph: &Graph, n: usize) -> Option<Vec<Path>> {
    let mut accesses = vec![None; graph.nodes().len()];
    let mut active_nodes = VecDeque::new();

    let mut paths: Vec<Path> = Vec::new();
    let mut id = graph.start();

    accesses[id.0] = Some(id);
    loop {
        if id == graph.end() {
            paths.push(trace_back(graph, &accesses));
            if paths.len() >= n {
                return Some(paths);
            }
        }
        for link in &graph[id].links {
            let access = &mut accesses[link.0];
            if access.is_none() {
                *access = Some(id);
                active_nodes.push_back(link);
            }
        }
        // End condition
        id = *active_nodes.pop_front()?;
    }
}

#[cfg(test)]
mod benches {
    extern crate test;
    use test::bench::Bencher;
    use crate::Graph;

    #[bench]
    fn shortest_path_for_map_1(b: &mut Bencher) {
        let graph = include_str!("../../maps/1").parse().unwrap();
        b.iter(|| super::shortest_path(&graph));
    }

    #[bench]
    fn shortest_path_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let graph = Graph::random(rng, 4_000, 0.10, 10);
        b.iter(|| super::shortest_path(&graph));
    }
}