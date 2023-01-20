use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BranchId(usize);

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