mod explorer;

use std::collections::VecDeque;

use crate::{BitArray, Graph, Path};

use explorer::{Branch, Explorer};

struct ValidPath {
    branch: Branch,
    hit_node: BitArray,
    incompats: BitArray,
}

struct WorkQueue {
    max_overlap: usize,
    queues: Vec<VecDeque<Branch>>,
}

impl WorkQueue {
    fn next(&mut self) -> Option<Branch> {
        for queue in &mut self.queues {
            match queue.pop_front() {
                Some(branch) => return Some(branch),
                None => {}
            }
        }
        None
    }

    fn new(max_overlap: usize) -> Self {
        Self {
            max_overlap,
            queues: Vec::new(),
        }
    }

    fn push(&mut self, branch: Branch, explorer: &Explorer) {
        let i = explorer[branch.node].len();
        if i >= self.max_overlap {
            return;
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

impl Graph {
    // TODO: add find optimal

    // TODO: sort result Vec<_>
    pub fn n_shortest_paths(&self, n: usize) -> Option<Vec<Path>> {
        // TODO: find better way
        if n == 0 {
            return Some(Vec::new());
        }

        // TODO: move to graph
        let max_possible = self.simple_throughput_majorant();

        if n > max_possible {
            return None;
        }
        let mut work_queue = WorkQueue::new(2 * n);
        let mut explorer = Explorer::new(self);
        let mut valid_paths: Vec<ValidPath> = vec![];

        work_queue.push(explorer.start(self.start), &explorer);

        let group = loop {
            let branch = work_queue.next()?;
            if branch.node == self.end {
                let mut hit_node = BitArray::new(self.nodes.len());

                let mut incompats = BitArray::new(valid_paths.len());
                for id in explorer.bracktrace(branch).skip(1) {
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

            for &dest in &self[branch.node].links {
                // TODO factorize repeated backtracing...
                if explorer.bracktrace(branch).all(|x| x != dest) {
                    work_queue.push(explorer.branch(branch, dest), &explorer);
                }
            }
        };
        Some(
            group
                .into_iter()
                .map(|branch| explorer.bracktrace(branch).skip(1).collect())
                .collect(),
        )
    }
}

#[cfg(test)]
mod benches {
    extern crate test;
    use crate::Graph;
    use test::bench::Bencher;

    #[bench]
    fn shortest_2_paths_for_map_1(b: &mut Bencher) {
        let graph: Graph = include_str_abs!("/maps/handmade/subject_map")
            .parse()
            .unwrap();
        b.iter(|| graph.n_shortest_paths(2));
    }

    #[bench]
    fn shortest_2_paths_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let graph: Graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| graph.n_shortest_paths(2));
    }

    #[bench]
    fn shortest_10_paths_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let graph: Graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| graph.n_shortest_paths(10));
    }

    #[bench]
    fn shortest_100_paths_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let graph: Graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| graph.n_shortest_paths(100));
    }
}
