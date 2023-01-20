use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathId(usize);

impl fmt::Display for PathId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "B#{}", self.0)
    }
}

impl From<usize> for PathId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<PathId> for usize {
    fn from(id: Id) -> usize {
        id.0
    } 
}