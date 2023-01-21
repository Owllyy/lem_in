use core::fmt;

use crate::NodeId;
use super::Explorer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BranchId(usize);

impl BranchId {
    pub const ORIGIN: BranchId = BranchId(0);
}

impl fmt::Display for BranchId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P#{}", self.0)
    }
}

impl From<usize> for BranchId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<BranchId> for usize {
    fn from(id: BranchId) -> usize {
        id.0
    } 
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Branch { 
    pub id: BranchId,
    pub node: NodeId,
}

impl Branch {
    pub fn origin() -> Self {
        Self {
            id: BranchId::ORIGIN,
            // TODO: find a more elegant way
            // Here I put usize::MAX because should never be used
            node: usize::MAX.into(),
        }
    }

    pub fn is_origin(&self) -> bool {
        self.id == BranchId::ORIGIN
    }
}

impl Explorer {
    fn next_id(&mut self) -> BranchId {
        let result = self.current_id;
        self.current_id.0 += 1;
        result
    }

    pub fn start(&mut self, at: NodeId) -> Branch {
        Branch {
            id: self.next_id(),
            node: at,
        }
    }

    pub fn branch(&mut self, sub_branch: Branch, dest: NodeId) -> Branch {
        let branch = Branch {
            id: self.next_id(),
            node: dest,
        };
        self.get_mut(dest).insert(branch.id, sub_branch);
        branch
    }

    pub fn rewind(&self, branch: Branch) -> Option<Branch> {
        self[branch.node].get(&branch.id).cloned()
    }
}
