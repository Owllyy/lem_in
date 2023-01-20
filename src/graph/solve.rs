use core::fmt;

use crate::path::Path;
use super::Graph;

#[derive(Debug)]
struct Step {
    duration: usize,
    paths: Vec<Path>,
}

#[derive(Debug)]
pub struct Solution(Vec<Step>);

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for Step{duration, paths} in &self.0 {
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
