mod backtrace;
mod branch;

pub use backtrace::Backtrace;

use std::{collections::HashMap, ops::Index};

use crate::{Graph, NodeId};
pub use branch::{Branch, BranchId};

pub type Record = HashMap<BranchId, Branch>;

pub struct Explorer {
    records: Vec<Record>,
    current_id: BranchId,
}

impl Explorer {
    pub fn new(graph: &Graph) -> Self {
        let records = (0..graph.nodes().len()).map(|_| Record::new()).collect();
        let current_id = BranchId::from(1);
        Self {
            records,
            current_id,
        }
    }

    fn get_mut(&mut self, at: NodeId) -> &mut Record {
        &mut self.records[usize::from(at)]
    }
}

impl Index<NodeId> for Explorer {
    type Output = Record;

    fn index(&self, at: NodeId) -> &Self::Output {
        &self.records[usize::from(at)]
    }
}
    
