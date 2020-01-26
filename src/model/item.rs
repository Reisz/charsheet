use super::{FrontEnd, ValueId};
use std::collections::HashMap;

/// Represents all the ways a Value can be modified by an Item.
pub enum Modification {
    /// Add to or subtract from a value
    Add(i32),
    /// Multiply a value
    Multiply(f32),
    /// Change to a predefined value
    Change(i32),
}

/// "Equippable" item. Can be used to represent actual items, learnable skills, traits or other
/// conditionals.
pub struct Item {
    /// Front end data
    pub front_end: FrontEnd,

    pub(crate) modifications: HashMap<ValueId, Modification>,
}

impl Item {
    /// Create a new item.
    pub fn new(front_end: FrontEnd) -> Self {
        Self {
            front_end,

            modifications: HashMap::new(),
        }
    }
}
