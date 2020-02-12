use super::{Calculation, ValueId};

/// Represents a modification.
pub struct Modification {
    priority: u16,
    calculation: Calculation,
}

impl Modification {
    /// Create a new Modification
    pub fn new(priority: u16, calculation: Calculation) -> Self {
        Self {
            priority,
            calculation,
        }
    }

    pub(crate) fn set_value(&mut self, id: ValueId) {
        self.calculation.replace_with_value(id);
    }

    /// Retrive the calculation.
    pub fn calculation(&self) -> &Calculation {
        &self.calculation
    }

    /// Retrieve the priority.
    pub fn priority(&self) -> u16 {
        self.priority
    }
}
