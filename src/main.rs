use lem_in::graph::Graph;
use lem_in::path::Path;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or("/dev/stdin".to_owned());

    // TODO: remove unwrap()
    let input = std::fs::read_to_string(path).unwrap();

    let graph: Graph = match input.parse() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("ERROR: {e:?}");
            std::process::exit(1);
        }
    };
    // println!("start = {}, end = {}", graph.start(), graph.end());
    // for (id, node) in graph.nodes().iter().enumerate() {
    //     println!("{id} {node:?}");
    // }
    // println!("{:#?}", Path::shortest(&graph));

    println!("{:#?}", Path::n_shortest(&graph, 3));
}
