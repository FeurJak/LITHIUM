use super::{CallStack, id::CallStackId};
use crate::Location;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationNode {
    pub parent: Option<CallStackId>,
    pub children: Vec<CallStackId>,
    pub children_hash: FxHashMap<u64, CallStackId>,
    pub value: Location,
}

impl LocationNode {
    pub fn new(parent: Option<CallStackId>, value: Location) -> Self {
        LocationNode { parent, children: Vec::new(), children_hash: FxHashMap::default(), value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct LocationNodeDebugInfo {
    pub parent: Option<CallStackId>,
    pub value: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash)]
pub struct LocationTree {
    pub locations: Vec<LocationNodeDebugInfo>,
}

impl LocationTree {
    pub fn get_call_stack(&self, mut call_stack: CallStackId) -> CallStack {
        let mut result = Vec::new();
        while let Some(parent) = self.locations[call_stack.index()].parent {
            result.push(self.locations[call_stack.index()].value);
            call_stack = parent;
        }
        result.reverse();
        result
    }
}
