mod node_id;
mod link;
mod name;
mod node;
mod solve;

use core::fmt;
use std::{ops::Index, str::FromStr, error::Error};

pub use node_id::NodeId;
pub use link::LinkByName;
pub use name::{is_invalid_name_char, Name};
pub use node::Node;

use ParseError::*;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    start: NodeId,
    end: NodeId,
    ant_count: usize,
}

impl Graph {
    fn add_node(&mut self, node: Node) -> Result<(), ParseError> {
        if self.nodes.iter().any(|n| n.name == node.name) {
            return Err(DuplicateName(node.name));
        }
        self.nodes.push(node);
        Ok(())
    }

    fn link_by_name(&mut self, link: LinkByName) -> Result<(), LinkingError> {
        let a = self
            .nodes
            .iter()
            .position(|n| n.name == link.a)
            .ok_or(LinkingError::UnknownName(link.a))?;
        let b = self
            .nodes
            .iter()
            .position(|n| n.name == link.b)
            .ok_or(LinkingError::UnknownName(link.b))?;
        self.nodes[a].links.push(NodeId::from(b));
        self.nodes[b].links.push(NodeId::from(a));
        Ok(())
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn start(&self) -> NodeId {
        self.start
    }

    pub fn end(&self) -> NodeId {
        self.end
    }

    pub fn ant_count(&self) -> usize {
        self.ant_count
    }

    // #[cfg(test)]
    pub fn random(mut rng: impl rand::Rng, node_count: usize, link_density: f32, max_ant_count: usize) -> Self {
        Self {
            start: NodeId::from(rng.gen_range(0..node_count)),
            end: NodeId::from(rng.gen_range(0..node_count)),
            nodes: (0..node_count)
                .map(|id| Node {
                    name: Name::from_str(&id.to_string()).unwrap(),
                    pos: node::Position { x: 0, y: 0 },
                    links: (0..node_count)
                        .filter(|_| rng.gen::<f32>() < link_density)
                        .map(|id| NodeId::from(id))
                        .collect(),
                })
                .collect(),
            ant_count: rng.gen_range(0..max_ant_count),
        }
    }
}

impl Index<NodeId> for Graph {
    type Output = Node;
    fn index(&self, id: NodeId) -> &Self::Output {
        &self.nodes[usize::from(id)]
    }
}

impl FromStr for Graph {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let number_of_ants: usize = lines
            .next()
            .ok_or(MissingAnts)?
            .parse()
            // TODO change
            .map_err(|_| MissingAnts)?;

        let mut parsing_nodes: bool = true;

        let mut graph = Graph {
            ant_count: number_of_ants,
            start: NodeId::from(0),
            end: NodeId::from(0),
            nodes: vec![],
        };
        let mut start = None;
        let mut end = None;

        for line in lines {
            if line.starts_with("##") {
                let name = &line[2..];
                let next_node_id = graph.nodes.len();
                let previous_value = match name {
                    "start" => start.replace(next_node_id),
                    "end" => end.replace(next_node_id),
                    _ => return Err(InvalidTag(name.to_owned())),
                };
                if previous_value.is_some() {
                    return Err(DuplicateTag(name.to_owned()));
                }
                continue;
            } else if line.starts_with("#") {
                continue;
            }

            if parsing_nodes {
                match line.parse() {
                    Ok(node) => graph.add_node(node)?,
                    Err(_) => parsing_nodes = false,
                }
            }

            if !parsing_nodes {
                let link = line.parse()?;
                graph.link_by_name(link)?;
            }
        }

        let Some(start) = start else {
            return Err(MissingTag("start".to_owned()));
        };
        let Some(end) = end else {
            return Err(MissingTag("end".to_owned()));
        };
        graph.start = NodeId::from(start);
        graph.end = NodeId::from(end);

        Ok(graph)
    }
}

#[derive(Debug)]
pub enum ParseError {
    MissingAnts,
    DuplicateName(Name),
    InvalidTag(String),
    DuplicateTag(String),
    MissingTag(String),
    LinkParseError(link::ParseError),
    LinkingError(LinkingError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MissingAnts => write!(f, "Missing ant section"),
            DuplicateName(name) => write!(f, "Duplicate name {}", name.as_ref()),
            InvalidTag(tag) => write!(f, "Invalid tag {tag}"),
            DuplicateTag(tag)=> write!(f, "Duplicate tag {tag}"),
            MissingTag(tag) => write!(f, "Missing tag {tag}"),
            LinkParseError(link_error) => write!(f, "Could not parse link: {link_error}"),
            LinkingError(linking_error) => write!(f, "Invalid link: {linking_error}"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LinkingError(ref error) => Some(error),
            Self::LinkParseError(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<link::ParseError> for ParseError {
    fn from(error: link::ParseError) -> Self {
        Self::LinkParseError(error)
    }
}

impl From<LinkingError> for ParseError {
    fn from(error: LinkingError) -> Self {
        Self::LinkingError(error)
    }
}

#[derive(Debug)]
pub enum LinkingError {
    UnknownName(Name),
}

impl fmt::Display for LinkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkingError::UnknownName(name) => write!(f, "Unkown node name {}", name.as_ref()),
        }
    }
}

impl Error for LinkingError {}

#[cfg(test)]
mod tests {
    use super::*;
    use ParseError::MissingAnts;

    #[test]
    fn empty_graph() {
        let result = "".parse::<Graph>();

        assert!(matches!(result, Err(MissingAnts)));
    }

    extern crate test;
    use test::bench::Bencher;

    #[bench]
    fn graph_random_gen(b: &mut Bencher) {
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        b.iter(|| Graph::random(&mut rng, 4_000, 0.10, 10));
    }
}

