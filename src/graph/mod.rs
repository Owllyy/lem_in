mod id;
mod name;
mod node;
mod link;

use std::str::FromStr;

pub use id::Id;
pub use name::{Name, is_invalid_name_char};
pub use node::Node;
pub use link::LinkByName;

use ParseError::*;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    start: Id,
    end: Id,
    number_of_ants: usize,
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
        let a = self.nodes.iter().position(|n| n.name == link.a)
            .ok_or(LinkingError::UnknownName(link.a))?;
        let b = self.nodes.iter().position(|n| n.name == link.b)
            .ok_or(LinkingError::UnknownName(link.b))?;
        self.nodes[a].links.push(Id(b));
        self.nodes[b].links.push(Id(a));
        Ok(())
    }
}

impl FromStr for Graph {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let number_of_ants: usize = lines.next()
            .ok_or(MissingAnts)?
            .parse()
            // TODO change
            .map_err(|_| MissingAnts)?;

        let mut parsing_nodes: bool = true;

        let mut graph = Graph {
            number_of_ants,
            start: Id(0),
            end: Id(0),
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
        graph.start = Id(start);
        graph.end = Id(end);

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

#[cfg(test)]
mod tests {
    use super::*;
    use ParseError::*;

    #[test]
    fn empty_graph() {
        let result = "".parse::<Graph>();

        assert!(matches!(result, Err(MissingAnts)));
    }
}
