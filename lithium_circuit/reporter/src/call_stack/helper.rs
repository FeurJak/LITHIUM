use super::{
    CallStack,
    id::CallStackId,
    tree::{LocationNode, LocationNodeDebugInfo, LocationTree},
};
use crate::Location;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStackHelper {
    pub locations: Vec<LocationNode>,
}

impl Default for CallStackHelper {
    fn default() -> Self {
        let mut result = CallStackHelper { locations: Vec::new() };
        result.add_location_to_root(Location::dummy());
        result
    }
}

impl CallStackHelper {
    pub fn get_call_stack(&self, mut call_stack: CallStackId) -> CallStack {
        let mut result = Vec::new();
        while let Some(parent) = self.locations[call_stack.index()].parent {
            result.push(self.locations[call_stack.index()].value);
            call_stack = parent;
        }
        result.reverse();
        result
    }
    pub fn extend_call_stack(
        &mut self,
        mut call_stack: CallStackId,
        locations: &CallStack,
    ) -> CallStackId {
        for location in locations {
            call_stack = self.add_child(call_stack, *location);
        }
        call_stack
    }
    pub fn add_child(&mut self, call_stack: CallStackId, location: Location) -> CallStackId {
        let key = fxhash::hash64(&location);
        if let Some(result) = self.locations[call_stack.index()].children_hash.get(&key) {
            if self.locations[result.index()].value == location {
                return *result;
            }
        }
        let new_location = LocationNode::new(Some(call_stack), location);
        let key = fxhash::hash64(&new_location.value);
        self.locations.push(new_location);
        let new_location_id = CallStackId::new(self.locations.len() - 1);

        self.locations[call_stack.index()].children.push(new_location_id);
        self.locations[call_stack.index()].children_hash.insert(key, new_location_id);
        new_location_id
    }
    pub fn unwind_call_stack(&self, mut call_stack: CallStackId, mut len: usize) -> CallStackId {
        while len > 0 {
            if let Some(parent) = self.locations[call_stack.index()].parent {
                len -= 1;
                call_stack = parent;
            } else {
                break;
            }
        }
        call_stack
    }

    pub fn add_location_to_root(&mut self, location: Location) -> CallStackId {
        if self.locations.is_empty() {
            self.locations.push(LocationNode::new(None, location));
            CallStackId::root()
        } else {
            self.add_child(CallStackId::root(), location)
        }
    }
    pub fn get_or_insert_locations(&mut self, locations: &CallStack) -> CallStackId {
        self.extend_call_stack(CallStackId::root(), locations)
    }
    pub fn to_location_tree(&self) -> LocationTree {
        LocationTree {
            locations: self
                .locations
                .iter()
                .map(|node| LocationNodeDebugInfo { value: node.value, parent: node.parent })
                .collect(),
        }
    }
}
