#[derive(Debug)]
pub struct Id(pub usize);

impl From<usize> for Id {
    fn from(id: usize) -> Self {
        Self(id)
    }
}
