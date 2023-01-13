mod backtrace;
use backtrace::Backtrace;

use super::Path;
use crate::{BitArray, Graph, Id};
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

fn find_group(
    incompats: &BitArray,
    paths: &[ValidPath],
    start: usize,
    count: usize,
) -> Option<Vec<PathId>> {
    if count == 0 {
        return Some(Vec::new());
    }
    for (path_index, path) in paths.iter().enumerate().skip(start) {
        if incompats.get(path_index) {
            continue;
        }

        let result = find_group(
            &(incompats | &path.incompats),
            paths,
            path_index + 1,
            count - 1,
        );

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
        let path_origin = path_id_generator.next();
        // Here I put usize::MAX because should never be used
        accesses[graph.start().0].insert(path_origin, (usize::MAX, Id(usize::MAX)));
        active_branches.push_back((path_origin, graph.start()));

        let group = loop {
            let (path_id, id) = active_branches.pop_front()?;
            if id == graph.end() {
                let mut hit_node = BitArray::new(graph.nodes().len());

                let mut incompats = BitArray::new(valid_paths.len());
                for id in Backtrace::new(graph, &accesses, path_id, id).skip(1) {
                    hit_node.add(id.0);
                    for (path_index, path) in valid_paths.iter().enumerate() {
                        incompats.add_if(path_index, path.hit_node.get(id.0));
                    }
                }

                if let Some(mut group) = find_group(&incompats, &valid_paths, 0, n - 1) {
                    group.push(path_id);
                    break group;
                }

                valid_paths.push(ValidPath {
                    path_id,
                    hit_node,
                    incompats,
                });
                continue;
            }

            for &link in &graph[id].links {
                // TODO factorize repeated backtracing...
                if link == graph.start() || Backtrace::new(graph, &accesses, path_id, id).any(|x| x == link)
                {
                    continue;
                }
                let new_path_id = path_id_generator.next();
                accesses[link.0].insert(new_path_id, (path_id, id));
                active_branches.push_back((new_path_id, link));
            }
        };
        Some(
            group
                .into_iter()
                .map(|path_id| {
                    Backtrace::new(graph, &accesses, path_id, graph.end())
                        .skip(1)
                        .collect()
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod benches {
    extern crate test;
    use super::Path;
    use crate::Graph;
    use test::bench::Bencher;

    #[bench]
    fn shortest_2_paths_for_map_1(b: &mut Bencher) {
        let graph = include_str!("../../../maps/1").parse().unwrap();
        b.iter(|| Path::n_shortest(&graph, 2));
    }

    #[bench]
    fn shortest_2_paths_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| Path::n_shortest(&graph, 2));
    }

    #[bench]
    fn shortest_10_paths_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| Path::n_shortest(&graph, 10));
    }

    #[bench]
    fn shortest_100_paths_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| Path::n_shortest(&graph, 100));
    }
}
