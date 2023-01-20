use super::Path;
use crate::Graph;
use std::collections::VecDeque;

impl Graph {
    pub fn shortest_path(&self) -> Option<Path> {
        let mut accesses = vec![None; self.nodes().len()];
        let mut active_nodes = VecDeque::new();

        let mut id = self.start();
        accesses[usize::from(id)] = Some(id);
        loop {
            if id == self.end() {
                break;
            }
            for link in &self[id].links {
                let access = &mut accesses[usize::from(id)];
                if access.is_none() {
                    *access = Some(id);
                    active_nodes.push_back(link);
                }
            }
            id = *active_nodes.pop_front()?;
        }

        let mut path = vec![id];
        while id != self.start() {
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
    use crate::Graph;
    use test::bench::Bencher;

    #[bench]
    fn shortest_path_for_map_1(b: &mut Bencher) {
        let graph: Graph = include_str_abs!("/maps/handmade/subject_map").parse().unwrap();
        b.iter(|| graph.shortest_path());
    }

    #[bench]
    fn shortest_path_random(b: &mut Bencher) {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(0);
        let graph = Graph::random(rng, 4_000, 0.001, 10);
        b.iter(|| graph.shortest_path());
    }
}
