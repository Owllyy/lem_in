use super::Path;
use crate::Graph;
use std::collections::VecDeque;

impl Path {
    pub fn shortest(graph: &Graph) -> Option<Self> {
        let mut accesses = vec![None; graph.nodes().len()];
        let mut active_nodes = VecDeque::new();

        let mut id = graph.start();
        accesses[usize::from(id)] = Some(id);
        loop {
            if id == graph.end() {
                break;
            }
            for link in &graph[id].links {
                let access = &mut accesses[usize::from(id)];
                if access.is_none() {
                    *access = Some(id);
                    active_nodes.push_back(link);
                }
            }
            id = *active_nodes.pop_front()?;
        }

        let mut path = vec![id];
        while id != graph.start() {
            id = accesses[usize::from(id)].unwrap();
            path.push(id);
        }
        path.reverse();
        Some(Path(path))
    }
}

#[cfg(test)]
mod benches {
    extern crate test;
    use super::Path;
    use crate::Graph;
    use test::bench::Bencher;

    #[bench]
    fn shortest_path_for_map_1(b: &mut Bencher) {
        let graph = include_str!("../../maps/handmade/subject_map").parse().unwrap();
        b.iter(|| Path::shortest(&graph));
    }

    #[bench]
    fn shortest_path_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| Path::shortest(&graph));
    }
}
