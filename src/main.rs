use core::{panicking::panic, num::dec2flt::parse};
use std::default;

struct Node {
    id: String,
    pos: (u32, u32),
    links: Vec<u32>,
}

struct Graph {
    nodes: Vec<Node>,
}

fn parse_node(line: &str) -> Option<Node> {
        let splited: Vec<_> = line.split(' ').collect();
        if splited.len() != 3 && splited[1..].iter().any(|s| s.parse::<u32>().is_err()) {
            parsing_nodes = false;
        }
        todo!();
}


fn main() {
    let input = include_str!("../maps/1");
    let mut lines = input.lines();
    let number_of_ants: u32 = lines.next().unwrap().parse().unwrap();
    let parsing_nodes: bool = true;

    let mut nodes = Vec::new();
    for line in lines {
        if line.starts_with("##") {
            let name = &line[2..];
            match name {
                "start" => todo!(),
                "end" => todo!(),
                _ => panic!("Invalid input"),
            }
            continue;
        } else if line.starts_with("#") {
            continue;
        }

        if parsing_nodes {
            match parse_node(line) {
                Some(node) => nodes.push(node),
                None => parsing_nodes = false,
            }
        }
        if !parsing_nodes {
            let splited: Vec<_> = line.split('-').collect();
            if splited.len() != 2 {
                panic!("Invalid input");
            }

        }
            
        }

    }
}
