mod explorer;

use std::{collections::VecDeque, ops::Index};

use crate::{BitArray, Graph, Path, Node, NodeId, graph::node};

use explorer::{Branch, Explorer};

struct ValidPath {
    branch: Branch,
    hit_node: BitArray,
    incompats: BitArray,
}

struct WorkQueue {
    max_overlap: usize,
    queues: Vec<VecDeque<Pathing>>,
}

#[derive(Clone, Debug)]
struct ValidPathing {
    nodes: Vec<NodeId>,
    hit_node: BitArray,
    incompats: BitArray,
}

#[derive(Clone, Debug)]
struct Pathing {
    nodes: Vec<NodeId>,
    timing: usize,
}

impl Pathing {
    fn next(&self) -> NodeId {
        *self.nodes.last().unwrap()
    }
}

impl WorkQueue {
    fn next(&mut self) -> Option<Pathing> {
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

    fn push(&mut self, pathing: Pathing) {
        match self.queues.get_mut(pathing.timing) {
            Some(queue) => queue.push_back(pathing),
            None => self.queues.push(VecDeque::from([pathing])),
        }
    }
}

fn find_group(
    incompats: &BitArray,
    paths: &Vec<ValidPathing>,
    start: usize,
    count: usize,
) -> Option<Vec<ValidPathing>> {
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
            start + 1,
            count - 1,
        );
        if let Some(mut group) = result {
            group.push(paths[path_index].clone());
            // if start == 0 && group.len() >= count {
                return Some(group);
            // }
        }
    }
    None
}

impl Graph {
    pub fn n_shortest_paths(&mut self, mut n: usize) -> Option<Vec<Path>> {

        // Check N
        if n == 0 {
            return Some(Vec::new());
        }
        let max_possible = self.simple_throughput_majorant();
        if n > max_possible {
            return None;
        }

        // Init workQ
        let mut work_queue = WorkQueue::new(2 * n);
        let mut valid_paths: Vec<ValidPathing> = Vec::new();
        let mut first_pathing: Pathing = Pathing {
            nodes : Vec::new(),
            timing : 0,
        };
        first_pathing.nodes.push(self.start());
        work_queue.push(first_pathing);

        let group: Vec<ValidPathing> = loop {
            let patho = work_queue.next();
            let mut pathing: Pathing;

            // Debug
            match patho {
                None => {for path in valid_paths { println!("NOT FOUND\n{} {}", path.hit_node, path.incompats); }return None;},
                Some(patho) => pathing = patho,
            }


            if pathing.next() == self.end {
                let mut hit_node = BitArray::new(self.nodes.len());
                let mut incompats = BitArray::new(valid_paths.len());
                
                // Set Hits
                for node in &pathing.nodes {
                    if node != &self.start() && node != &self.end() {
                        hit_node.set(usize::from(*node), true);
                    }
                }

                // Set Incompatibility
                for (i, path) in valid_paths.iter().enumerate() {
                    let new_hit = &hit_node & &path.hit_node;
                    for j in 0..self.nodes().len() {
                        if new_hit.get(j) {
                            incompats.add(i);
                            break;
                        }
                    }
                }
                
                // Find group
                if let Some(mut group) = find_group(&incompats, &valid_paths, 0, n - 1) {
                    let current = ValidPathing {
                        nodes: pathing.nodes.clone(),
                        hit_node,
                        incompats,
                    };
                    group.push(current);
                    break group;
                }

                // Push to Valid_path
                let current_valid_path = ValidPathing {
                    nodes: pathing.nodes.clone(),
                    hit_node,
                    incompats,
                };
                valid_paths.push(current_valid_path);

                continue;
            } else {

                // Set timing on path and path_counter on node
                if &pathing.timing < &self[pathing.next()].path_counter {
                    pathing.timing += 1;
                }
                self[pathing.next()].add_path_counter();

                // Push on Queue
                'test: for link in &self.index(pathing.next()).links {

                    // Skip Loop and Self
                    for node in &pathing.nodes {
                        if link == node {
                            continue 'test;
                        }
                    }

                    // Clone and Push
                    let mut clone = pathing.clone();
                    clone.nodes.push(link.to_owned());
                    work_queue.push(clone);
                }
            }
        };

        
        // Debug
        for path in valid_paths {
            println!("res : {:#?}", path.nodes);
        }

        // Vec<ValidPathing> to Vec<Path>
        let mut res: Vec<Path> = Vec::new();
        for path in group {
            res.push(Path{
                0: path.nodes.clone(),
            });
        }
        Some(res)
    }
}

#[cfg(test)]
mod benches {
    extern crate test;
    use crate::Graph;
    use test::bench::Bencher;
    #[test]
    fn try_it() {
        let mut graph: Graph = include_str_abs!("/maps/handmade/three_route")
            .parse()
            .unwrap();
        graph.n_shortest_paths(graph.simple_throughput_majorant());
    }

    #[bench]
    fn shortest_2_paths_for_map_1(b: &mut Bencher) {
        let mut graph: Graph = include_str_abs!("/maps/handmade/subject_map")
            .parse()
            .unwrap();
        b.iter(|| graph.n_shortest_paths(100));
    }

    #[bench]
    fn shortest_2_paths_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut graph: Graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| graph.n_shortest_paths(2));
    }

    #[bench]
    fn shortest_10_paths_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut graph: Graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| graph.n_shortest_paths(10));
    }

    #[bench]
    fn shortest_100_paths_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut graph: Graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| graph.n_shortest_paths(100));
    }
}
