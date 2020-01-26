use super::{FrontEnd, ItemId, ValueId};

/// A value in the character sheet.
pub struct Value {
    /// Front end data
    pub front_end: FrontEnd,

    pub(crate) default: i32,

    pub(crate) dependencies: Vec<(f32, ValueId)>,
    pub(crate) modifying_items: Vec<ItemId>,
    pub(crate) dependents: Vec<ValueId>,
}

impl Value {
    /// Create a new value.
    pub fn new(front_end: FrontEnd, default: i32) -> Self {
        Self {
            front_end,
            default,

            dependencies: Vec::new(),
            modifying_items: Vec::new(),
            dependents: Vec::new(),
        }
    }
}
