use core::fmt;
use std::{collections::VecDeque, io};

use super::Graph;
use crate::path::Path;

#[derive(Debug)]
struct Step {
    duration: usize,
    paths: Vec<Path>,
}

#[derive(Debug)]
pub struct Solution(Vec<Step>);

fn cycle(vec: &mut VecDeque<Option<usize>>, x: Option<usize>) {
    vec.pop_back();
    vec.push_front(x);
}

// Print to the correct format
fn print(
    output: &mut impl io::Write,
    path: &Path,
    ant_vec: &VecDeque<Option<usize>>,
) -> io::Result<()> {
    for (node, ant) in path.as_ref().iter().zip(ant_vec) {
        if let Some(ant) = ant {
            write!(output, "L{}-{} ", ant, usize::from(*node))?;
        }
    }
    Ok(())
}

impl Solution {
    pub fn write_to(&self, mut output: impl io::Write) -> io::Result<()> {
        // Skip ahead & prevent panic
        let Some(all_path_step) = self.0.get(0) else {
            return Ok(());
        };

        // Prefill with None
        let mut ant_vecs: Vec<VecDeque<Option<usize>>> = all_path_step
            .paths
            .iter()
            .map(|_| (0..all_path_step.paths.len()).map(|_| None).collect())
            .collect();

        // Id generator
        let mut next_ant_id = {
            let mut id = 0;
            move || {
                let result = id;
                id += 1;
                result
            }
        };

        // Push In and Forward ant in Paths + Print
        for current_step in &self.0 {
            for _ in 0..current_step.duration {
                for (i, (path, ants)) in all_path_step.paths.iter().zip(&mut ant_vecs).enumerate() {
                    cycle(ants, current_step.paths.get(i).map(|_| next_ant_id()));
                    print(&mut output, path, ants)?;
                }
                write!(output, "\n")?;
            }
        }

        // Push Forward remaining ant in Paths + Print
        let mut has_ants = true;
        while has_ants {
            has_ants = false;
            for (path, ants) in all_path_step.paths.iter().zip(&mut ant_vecs) {
                cycle(ants, None);
                print(&mut output, path, ants)?;
            }
            if !ant_vecs.iter().all(|ants| ants.iter().all(|x| x.is_none())) {
                has_ants = true;
                write!(output, "\n")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for Step { duration, paths } in &self.0 {
            writeln!(f, "{duration} times:")?;
            for path in paths {
                writeln!(f, "    - {path:?}")?;
            }
        }
        Ok(())
    }
}

impl Graph {
    pub fn simple_throughput_majorant(&self) -> usize {
        let start_link_count = self[self.start].links.len();
        let end_link_count = self[self.start].links.len();
        start_link_count.min(end_link_count)
    }

    pub fn solve(&self) -> Option<Solution> {
        // TODO: use better one found on the fly
        let mut n = self.simple_throughput_majorant();
        let mut paths = loop {
            if let Some(paths) = Path::n_shortest(self, n) {
                break paths;
            }
            n -= 1;
            if n == 0 {
                return None;
            }
        };
        // TODO: get the paths to be pre sorted
        paths.sort_by_key(Path::len);

        let mut steps = Vec::new();
        let mut remaining_ants = self.ant_count;
        let mut used_path = &paths[..];
        while let Some((longest, others)) = used_path.split_last() {
            let min_required_ants: usize = others.iter().map(|p| longest.len() - p.len()).sum();
            if let Some(rest) = remaining_ants.checked_sub(min_required_ants) {
                let duration = rest / used_path.len();
                if duration != 0 {
                    steps.push(Step {
                        duration,
                        paths: used_path.to_owned(),
                    });
                    remaining_ants -= duration * used_path.len();
                }
            }
            used_path = others;
        }
        Some(Solution(steps))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_move_ants() {
        let graph: Graph = include_str!("../../maps/generated/big_superposition/0").parse().unwrap();

        let Some(solution) = graph.solve() else {
            // Nothing to check
            return;
        };

        // Get output
        let mut output = Vec::new();
        solution.write_to(&mut output);

        // Check
        for (line_number, line) in output.split(|&c| c == b'\n').enumerate() {
            let movements: Vec<_> = line.split(|&c| c == b' ').filter(|&m| !m.is_empty()).map(|m| {
                let mut parts = m[1..].split(|&c| c == b'-');
                let ant_id = parts.next().unwrap();
                let node_id = parts.next().unwrap();
                (ant_id, node_id)
            }).collect();
            
            for (i, &(ant, node)) in movements.iter().enumerate() {
                for &(ant2, node2) in movements[i..].iter().skip(1)  {
                    let ant_str = std::str::from_utf8(ant2).unwrap();
                    let node_str = std::str::from_utf8(node2).unwrap();
                    assert!(ant != ant2, "Duplicate ant on line {line_number}: {ant_str}");
                    assert!(node != node2, "Duplicate node on line {line_number}: {node_str}");
                }
            }
        }
    }
}