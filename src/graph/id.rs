use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(usize);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl From<usize> for Id {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<Id> for usize {
    fn from(id: Id) -> usize {
        id.0
    } 
}