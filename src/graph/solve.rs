use core::fmt;
use std::{io, collections::VecDeque, fmt::write};

use crate::path::Path;
use super::Graph;

#[derive(Debug)]
struct Step {
    duration: usize,
    paths: Vec<Path>,
}

#[derive(Debug)]
pub struct Solution(Vec<Step>);

fn push_bounded(vec: &mut VecDeque<Option<usize>>, x: Option<usize>, limit: usize) {
    vec.push_front(x);
    if vec.len() > limit {
        vec.pop_back();
    }
}

impl Solution {
    pub fn write_to(&self, mut output: impl io::Write) -> io::Result<()> {
        let mut ant_id: usize = 0;
        let mut ant_vec: Vec<VecDeque<Option<usize>>> = Vec::new();

        fn print(output: &mut impl io::Write, path: &Path, ant_vec: &VecDeque<Option<usize>>) -> io::Result<()> {
            for (j, node) in path.as_ref().iter().enumerate() {
                if ant_vec.len() > j {
                    if let Some(ant) = ant_vec[j] {
                        write!(output, "L{}-{} ", ant, usize::from(*node))?;
                    }
                }
            }
            Ok(())
        }

        for (step_index, current_step) in self.0.iter().enumerate() {
            for path in &current_step.paths {
                ant_vec.push(VecDeque::new());
            }

            let Some(latency) = current_step.paths.iter().map(|e| e.len()).max() else {
                // TODO choose behaviour for now we are lazy
                return Ok(())
            };
            
            
            for time in 0..current_step.duration {
                for (i, path) in self.0[0].paths.iter().enumerate() {
                    // push ant in current_step scope and push forward others
                    if current_step.paths.len() > i {
                        push_bounded(&mut ant_vec[i], Some(ant_id), path.len());
                    } else {
                        push_bounded(&mut ant_vec[i], None, path.len());
                    }
                    ant_id += 1;
                    print(&mut output, path, &mut ant_vec[i])?;
                }
                write!(output, "\n")?;
            }
    }
    let Some(latency) = self.0[0].paths.iter().map(|e| e.len()).max() else {return Ok(())};
    for i in 0..latency {
        for (i, path) in self.0[0].paths.iter().enumerate() {
            push_bounded(&mut ant_vec[i], None, path.len());
            print(&mut output, path, &ant_vec[i])?;
        }
        write!(output, "\n")?;
    }
    Ok(())
    }
}

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
