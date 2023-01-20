mod shortest;
mod n_shortest;

use crate::NodeId;

#[derive(Debug, Clone)]
pub struct Path(Vec<NodeId>);

impl FromIterator<NodeId> for Path {
    fn from_iter<T: IntoIterator<Item = NodeId>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl AsRef<[Id]> for Path {
    fn as_ref(&self) -> &[Id] {
        &self.0
    }
}

impl Path {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
