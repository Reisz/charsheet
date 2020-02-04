use super::{Calculation, IntoCalculation};

/// Represents an inventory type.
#[derive(Default)]
pub struct Inventory {
    pub(crate) capacity: Option<Calculation>,
    pub(crate) slots: Option<Calculation>,
}

impl Inventory {
    /// Create a new inventory type.
    pub fn new() -> Self {
        Self::default()
    }

    /// Limit capacity to the result of the supplied calculation.
    pub fn capacity(mut self, calc: impl IntoCalculation) -> Self {
        self.capacity = Some(calc.into_calc());
        self
    }

    /// Limit slots to the result of the supplied calculation.
    pub fn slots(mut self, calc: impl IntoCalculation) -> Self {
        self.slots = Some(calc.into_calc());
        self
    }
}
