use std::io::Read;
use std::fs::File;
use lem_in::graph::Graph;

const RANDOM_GRAPH_NODE_COUNT: usize = 4_000;
const RANDOM_GRAPH_DENSITY: f32 = 0.001;
const RANDOM_GRAPH_MAX_ANT_COUNT: usize = 1_000;

fn random_graph() -> Graph {
    use rand::SeedableRng;
    let rng = rand::rngs::StdRng::seed_from_u64(1);
    Graph::random(
        rng,
        RANDOM_GRAPH_NODE_COUNT,
        RANDOM_GRAPH_DENSITY,
        RANDOM_GRAPH_MAX_ANT_COUNT,
    )
}

fn show_graph_stats(graph: &Graph) {
    println!("Start: {}", graph.start());
    println!("End: {}", graph.end());
    println!("Ant count: {}", graph.ant_count());
}

fn load_graph(mut input: impl Read) -> Result<Graph, String> {
    let mut content = String::new();
    input.read_to_string(&mut content)
        .map_err(|e| format!("Error reading file: {e}"))?;
    content.parse()
        .map_err(|e| format!("Invalid map: {e}"))
}

fn get_graph() -> Result<Graph, String> {
    let arg = std::env::args().nth(1);
    match arg.as_deref() {
        Some(arg) if arg == "--random" => {
            eprintln!("Generating random map (dens = {}%)...",
                RANDOM_GRAPH_DENSITY * 100.0
            );
            let graph = random_graph();
            show_graph_stats(&graph);
            Ok(graph)
        },
        Some(path) => {
            let file = File::open(path)
                .map_err(|e| format!("Could not read file: {e}"))?;
            eprintln!("Loading file {path}...");
            load_graph(file)
        }
        None => {
            eprintln!("Loading stdin...");
            load_graph(std::io::stdin())
        }
    }
}

fn run() -> Result<(), String> {
    let graph = get_graph()?;
    match graph.solve() {
        Some(solution) => solution.write_to(std::io::stdout())
             .map_err(|e| format!("Could write to stdout: {e}")),
        None => {
            println!("No solution was found");
            Ok(())
        }
    }
}

fn main() {
    if let Err(err) = run() {
        println!("Error:\n    {err}");
        std::process::exit(1);
    }
}
