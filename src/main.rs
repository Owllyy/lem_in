use lem_in::graph::Graph;

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
    println!("{graph:#?}");
}
