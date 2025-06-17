use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CallStackId(u32);

impl CallStackId {
    pub fn root() -> Self {
        Self::new(0)
    }

    pub fn new(id: usize) -> Self {
        Self(id as u32)
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn is_root(&self) -> bool {
        self.0 == 0
    }
}
