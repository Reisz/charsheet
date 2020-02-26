use super::{Calculation, FrontEnd, ItemId, ValueId};

/// A value in the character sheet.
pub struct Value {
    /// Front end data
    pub front_end: Option<FrontEnd>,

    pub(crate) default: i32,

    pub(crate) dependencies: Vec<Calculation>,
    pub(crate) modifying_items: Vec<ItemId>,
    pub(crate) dependents: Vec<ValueId>,
    pub(crate) conditions: Vec<ItemId>,
}

impl Value {
    /// Create a new value.
    pub fn new(default: i32) -> Self {
        Self {
            front_end: None,
            default,

            dependencies: Vec::new(),
            modifying_items: Vec::new(),
            dependents: Vec::new(),
            conditions: Vec::new(),
        }
    }

    /// Allow this item to be front-end visible.
    pub fn frontend(mut self, front_end: FrontEnd) -> Self {
        self.front_end = Some(front_end);
        self
    }
}
