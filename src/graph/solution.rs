use core::fmt;
use std::io;
use crate::Path;
use crate::Graph;

#[derive(PartialEq, Eq)]
pub struct SolutionStep {
	duration: usize,
	paths: Vec<Path>,
}

impl Graph { 
    fn get_latency(paths: &Vec<Path>) -> usize{
        let mut size = 0;
        for y in paths {
            let current_size = y.iter().count();
            if size < current_size {
                size = current_size;
            }
        }
        size
    }

    pub fn solve(&self) -> Option<Solution> {

        let mut result = Solution{
            0: Vec::new(),
        };
        let mut flow = self.find_max_valid_path();
        let Some(x) = Path::n_shortest(self, flow) else {return None};
        let mut ant_left = self.ant_count;
        let mut latency = Graph::get_latency(&x);
        result.0.push(SolutionStep { duration: (ant_left - latency) / latency, paths: x.clone() });

        ant_left = latency;

        loop {
            flow -= 1;
            let Some(next) = Path::n_shortest(&self, flow) else {
                continue;
            };
            let next_latency = Graph::get_latency(&next);
            if next_latency >= latency {
                continue ;
            } else if flow == 0 {
                break;
            }
            latency = next_latency;
            result.0.push(SolutionStep { duration: (ant_left - latency) / latency, paths: next.clone() });
            ant_left = latency;
        }

        Some(result)
    }
}

#[derive(PartialEq, Eq)]
pub struct Solution(Vec<SolutionStep>);

impl Solution {
	fn write_to(&self, output: impl io::Write) -> io::Result<()> {
		todo!();
	}
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for step in &self.0 {
            write!(f, "{}\n", step.duration)?;
            for path in &step.paths {
                for node in path.iter() {
                    write!(f, "{} ", node)?;
                }
            }
            write!(f, "/ ")?;
            write!(f, "\n\n")?;
        }
        Ok(())
    }
}