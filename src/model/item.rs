use super::{Calculation, FrontEnd, Id, IntoCalculation, Inventory, Modification, Value};
use std::{collections::HashMap, num::NonZeroU16};

pub(crate) struct Physical {
    pub(crate) size: u16,
    pub(crate) stack_size: NonZeroU16,
}

/// "Equippable" item. Can be used to represent actual items, learnable skills, traits or other
/// conditionals.
#[derive(Default)]
pub struct Item {
    /// Front end data
    pub front_end: Option<FrontEnd>,

    pub(crate) physical: Option<Physical>,
    pub(crate) has_inventory: Option<Id<Inventory>>,

    pub(crate) condition: Option<Calculation>,
    pub(crate) modifications: HashMap<Id<Value>, Modification>,
}

impl Item {
    /// Create a new item.
    pub fn new() -> Self {
        Self::default()
    }

    /// Allow this item to be front-end visible.
    pub fn frontend(mut self, front_end: FrontEnd) -> Self {
        self.front_end = Some(front_end);
        self
    }

    /// The item will be automatically applied based on the given condition.
    pub fn set_condition(mut self, condition: impl IntoCalculation) -> Self {
        self.condition = Some(condition.into_calc());
        self
    }

    /// Declare this to be a physical item that can be put into inventories.
    pub fn set_physical(mut self, size: u16, stack_size: u16) -> Self {
        let stack_size = NonZeroU16::new(stack_size).unwrap();
        self.physical = Some(Physical { size, stack_size });
        self
    }

    /// Change the invetory type to use with this item.
    pub fn set_inventory(mut self, id: Id<Inventory>) -> Self {
        self.has_inventory = Some(id);
        self
    }
}
