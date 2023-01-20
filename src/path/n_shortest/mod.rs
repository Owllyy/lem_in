mod backtrace;
mod branch_id;
use backtrace::Backtrace;

use super::Path;

use branch_id::BranchId;
use crate::{BitArray, Graph, Id};
use std::collections::{HashMap, VecDeque};

struct ValidPath {
    branch: Branch,
    hit_node: BitArray,
    incompats: BitArray,
}

struct BranchGenerator(BranchId);

#[derive(Clone, Copy)]
pub struct Branch { 
    id : BranchId,
    node: Id,
}

impl BranchGenerator {
    fn new() -> Self {
        Self(0.into())
    }

    fn next(&mut self) -> BranchId {
        let result = self.0;
        self.0 = (usize::from(self.0) + 1).into();
        result
    }

    pub fn create(&mut self, node: Id) -> Branch {
        Branch {
            id: self.next(),
            node,
        }
    }
}

struct WorkQueue {
    max_overlap: usize,
    queues : Vec<VecDeque<Branch>>,
}

type AccessRecord = HashMap<BranchId, Branch>;

impl WorkQueue {
    fn next(&mut self) -> Option<Branch> {
        for queue in &mut self.queues {
            match queue.pop_front() {
                Some(branch) => return Some(branch),
                None => {},
            }
        }
        None
    }

    fn new(max_overlap: usize) -> Self {
        Self {
            max_overlap,
            queues : Vec::new(),
        }
    }

    fn push(&mut self, branch: Branch, accesses: &[AccessRecord]) {
        let access = &accesses[usize::from(branch.node)];
        let i = access.len();
        if i >= self.max_overlap {
            return
        }
        match self.queues.get_mut(i) {
            Some(queue) => queue.push_back(branch),
            None => self.queues.push(VecDeque::from([branch])),
        }
    }
}

fn find_group(
    incompats: &BitArray,
    paths: &[ValidPath],
    start: usize,
    count: usize,
) -> Option<Vec<Branch>> {
    if paths[start..].len() < count {
        return None;
    }
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
            group.push(path.branch);
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
        // TODO: move to graph
        let max_possible = graph[graph.start()].links.len().min(graph[graph.end()].links.len());

        if n > max_possible {
            return None;
        }
        let mut work_queue = WorkQueue::new(2 * n);
        let mut accesses: Vec<_> = (0..graph.nodes().len()).map(|_| AccessRecord::new()).collect();
        let mut valid_paths: Vec<ValidPath> = vec![];

        let mut branch_generator = BranchGenerator::new();
        let branch_origin = branch_generator.next();
        // Here I put usize::MAX because should never be used
        accesses[usize::from(graph.start())].insert(branch_origin, branch_generator.create(Id::from(usize::MAX)));
        work_queue.push(branch_generator.create(graph.start()), &accesses);

        let group = loop {
            let branch = work_queue.next()?;
            if branch.node == graph.end() {
                let mut hit_node = BitArray::new(graph.nodes().len());

                let mut incompats = BitArray::new(valid_paths.len());
                for id in Backtrace::new(graph, &accesses, branch).skip(1) {
                    hit_node.add(usize::from(id));
                    for (path_index, path) in valid_paths.iter().enumerate() {
                        incompats.add_if(path_index, path.hit_node.get(usize::from(id)));
                    }
                }

                if let Some(mut group) = find_group(&incompats, &valid_paths, 0, n - 1) {
                    group.push(branch);
                    break group;
                }

                valid_paths.push(ValidPath {
                    branch,
                    hit_node,
                    incompats,
                });
                continue;
            }

            for &link in &graph[branch.node].links {
                // TODO factorize repeated backtracing...
                if link == graph.start() || Backtrace::new(graph, &accesses, branch).any(|x| x == link) {
                    continue;
                }
                let new_branch = branch_generator.create(link);
                accesses[usize::from(new_branch.node)].insert(new_branch.id, branch);
                work_queue.push(new_branch, &accesses);
            }
        };
        Some(
            group
                .into_iter()
                .map(|branch| {
                    Backtrace::new(graph, &accesses, branch)
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
