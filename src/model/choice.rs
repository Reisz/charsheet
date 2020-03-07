use super::{FrontEnd, Id, Modification, Value};
use std::collections::HashMap;

/// Part of a Choice.
pub struct Selection {
    pub(crate) front_end: Option<FrontEnd>,
    pub(crate) modifications: HashMap<Id<Value>, Modification>,
}

/// Represents a set of Selections. The Character will have to have exactly one Selection active at a time.
#[derive(Default)]
pub struct Choice {
    pub(crate) front_end: Option<FrontEnd>,
    pub(crate) options: Vec<Selection>,
}

impl Selection {
    /// Create a new selection.
    pub fn new(mods: impl Iterator<Item = (Id<Value>, Modification)>) -> Self {
        Self {
            front_end: None,
            modifications: mods.collect(),
        }
    }

    /// Add front end metadata.
    pub fn front_end(mut self, front_end: FrontEnd) -> Self {
        self.front_end = Some(front_end);
        self
    }
}

impl Choice {
    /// Create a new  Choice.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add front end metadata.
    pub fn front_end(mut self, front_end: FrontEnd) -> Self {
        self.front_end = Some(front_end);
        self
    }
}
