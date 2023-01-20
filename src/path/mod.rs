mod shortest;
mod n_shortest;

use crate::Id;

#[derive(Debug, Clone)]
pub struct Path(Vec<Id>);

impl FromIterator<Id> for Path {
    fn from_iter<T: IntoIterator<Item = Id>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Path {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
