use lem_in::graph::{Graph, self};
use lem_in::path::Path;

fn main() {
    // let path = std::env::args().nth(1)
    //     .unwrap_or("/dev/stdin".to_owned());

    // TODO: remove unwrap()
    // let input = std::fs::read_to_string(path).unwrap();

    // let graph: Graph = match input.parse() {
    //     Ok(g) => g,
    //     Err(e) => {
    //         eprintln!("ERROR: {e:?}");
    //         std::process::exit(1);
    //     } // };
    use rand::SeedableRng;
    let rng = rand::rngs::StdRng::seed_from_u64(0);
    let graph = Graph::random(rng, 4_590, 0.01, 5);
    let max_possible = graph[graph.start()].links.len().min(graph[graph.end()].links.len());
    // println!("start = {}, end = {}", graph.start(), graph.end());
    // for (id, node) in graph.nodes().iter().enumerate() {
    //     println!("{id} {node:?}");
    // }
    let start = std::time::Instant::now();
    println!("{:#?}", Path::n_shortest(&graph, max_possible));
    println!("in {:?}", start.elapsed());
}
