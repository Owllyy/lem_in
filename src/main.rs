use lem_in::graph::Graph;
use lem_in::path::Path;

fn main() {
    // let path = std::env::args().nth(1,)
    //     .unwrap_or("/dev/stdin".to_owned());

    // // TODO: remove unwrap()
    // let input = std::fs::read_to_string(path).unwrap();

    // let graph: Graph = match input.parse() {
    //     Ok(g) => g,
    //     Err(e) => {
    //         eprintln!("ERROR: {e:?}");
    //         std::process::exit(1,);
    //     }
    // };
    use rand::SeedableRng;
    let rng = rand::rngs::StdRng::seed_from_u64(1);
    let graph = Graph::random(rng, 4_000, 0.001, 41);
    println!("There are {} ants", graph.ant_count());

    if let Some(solution) = graph.solve() {
        solution.write_to(std::io::stdout());
        // println!("{}", solution);
    } else {
        println!("No solution found");
    }
    // println!("start = {}, end = {}", graph.start(), graph.end());
    // for (id, node) in graph.nodes().iter().enumerate() {
    //     println!("{id} {node:?}");
    // }
    // let start = std::time::Instant::now();
    // println!("{:#?}", Path::n_shortest(&graph, max_possible));
    // println!("in {:?}", start.elapsed());
}
