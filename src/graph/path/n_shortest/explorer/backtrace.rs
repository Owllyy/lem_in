use crate::NodeId;
use super::{Branch, Explorer};

pub struct Backtrace<'explorer> {
    current: Branch,
    explorer: &'explorer Explorer,
}

impl<'explorer> Iterator for Backtrace<'explorer> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.explorer.rewind(self.current).map(|b| {
            self.current = b;
            b.node
        })
    }
}

impl super::Explorer {
    pub fn bracktrace(&self, branch: Branch) -> Backtrace {
        Backtrace {
            current: branch,
            explorer: self,
        }
    }
}
