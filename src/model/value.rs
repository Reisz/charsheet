use super::{FrontEnd, ValueId};

/// A value in the character sheet.
pub struct Value {
    /// Front end data
    pub front_end: FrontEnd,

    pub(crate) base: i32,

    pub(crate) dependencies: Vec<(f32, ValueId)>,
    pub(crate) dependents: Vec<ValueId>,
}

impl Value {
    /// Create a new value.
    pub fn new(front_end: FrontEnd, base: i32) -> Self {
        Self {
            front_end,
            base,

            dependencies: Vec::new(),
            dependents: Vec::new(),
        }
    }
}
