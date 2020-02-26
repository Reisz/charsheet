use super::FrontEnd;

/// Part of a Choice.
pub struct Selection {
    pub(crate) front_end: Option<FrontEnd>,
    // pub(crate) modifications: HashMap<ValueId, Modification>,
}

/// Represents a set of Selections. The Character will have to have exactly one Selection active at a time.
pub struct Choice {
    pub(crate) front_end: Option<FrontEnd>,
    // pub(crate) options: Vec<Selection>,
}

impl Selection {
    /// Add front end metadata.
    pub fn front_end(mut self, front_end: FrontEnd) -> Self {
        self.front_end = Some(front_end);
        self
    }
}

impl Choice {
    /// Add front end metadata.
    pub fn front_end(mut self, front_end: FrontEnd) -> Self {
        self.front_end = Some(front_end);
        self
    }
}
