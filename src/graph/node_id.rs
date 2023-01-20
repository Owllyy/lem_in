use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeId(usize);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl From<usize> for NodeId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<NodeId> for usize {
    fn from(id: NodeId) -> usize {
        id.0
    } 
}