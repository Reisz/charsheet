use super::{Calculation, FrontEnd, IntoCalculation, ValueId};
use std::{collections::HashMap, num::NonZeroU16};

/// Represents all the ways a Value can be modified by an Item.
pub enum Modification {
    /// Add to or subtract from a value
    Add(i32),
    /// Multiply a value
    Multiply(f32),
    /// Change to a predefined value
    Change(i32),
}

pub(crate) struct InventoryInfo {
    pub(crate) size: u16,
    pub(crate) stack_size: NonZeroU16,
}

/// "Equippable" item. Can be used to represent actual items, learnable skills, traits or other
/// conditionals.
pub struct Item {
    /// Front end data
    pub front_end: FrontEnd,

    pub(crate) inventory_info: Option<InventoryInfo>,

    pub(crate) condition: Option<Calculation>,
    pub(crate) modifications: HashMap<ValueId, Modification>,
}

impl Item {
    /// Create a new item.
    pub fn new(front_end: FrontEnd) -> Self {
        Self {
            front_end,
            inventory_info: None,

            condition: None,
            modifications: HashMap::new(),
        }
    }

    /// Create a new item, which automatically applies itself based on the given condition.
    pub fn with_condition(front_end: FrontEnd, condition: impl IntoCalculation) -> Self {
        Self {
            front_end,
            inventory_info: None,

            condition: Some(condition.into_calc()),
            modifications: HashMap::new(),
        }
    }

    /// Declare this to be a physical item that can be put into inventories.
    pub fn allow_in_inventory(mut self, size: u16, stack_size: u16) -> Self {
        let stack_size = NonZeroU16::new(stack_size).unwrap();
        self.inventory_info = Some(InventoryInfo { size, stack_size });
        self
    }
}
