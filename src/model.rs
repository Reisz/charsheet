//! Allows defining rules for values and items.

mod calculation;
mod front_end;
mod inventory;
mod item;
mod value;

pub use calculation::*;
pub use front_end::*;
pub use inventory::*;
pub use item::*;
pub use value::*;

use std::collections::HashMap;

macro_rules! id {
    ($name:ident) => {
        /// Wrapper to prevent mixing ids
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(u32);

        impl $name {
            pub(crate) fn idx(self) -> usize {
                self.0 as usize
            }
        }
    };
}

id!(ValueId);
id!(InventoryId);
id!(ItemId);

/// Contains a set of values and items that can be used together.
#[derive(Default)]
pub struct Model {
    values: Vec<Value>,
    inventories: Vec<Inventory>,
    items: Vec<Item>,

    main_inventory: Option<InventoryId>,

    value_ids: HashMap<String, ValueId>,
    inventory_ids: HashMap<String, InventoryId>,
    item_ids: HashMap<String, ItemId>,
}

impl Model {
    /// Create a new Model.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new value to the model. Id string can not alias other value ids.
    pub fn add_value(&mut self, id_str: impl ToString, value: Value) -> ValueId {
        let id = ValueId(self.values.len() as u32);

        let id_str = id_str.to_string();
        assert!(self.value_ids.get(&id_str).is_none());
        self.value_ids.insert(id_str, id);

        self.values.push(value);
        id
    }

    /// Add a new inventory type.
    pub fn add_inventory(&mut self, id_str: impl ToString, inventory: Inventory) -> InventoryId {
        let id = InventoryId(self.inventories.len() as u32);

        let id_str = id_str.to_string();
        assert!(self.inventory_ids.get(&id_str).is_none());
        self.inventory_ids.insert(id_str, id);

        self.inventories.push(inventory);
        id
    }

    /// Add a new item to the model. Id string can not alias other item ids.
    pub fn add_item(&mut self, id_str: impl ToString, item: Item) -> ItemId {
        let id = ItemId(self.items.len() as u32);

        let id_str = id_str.to_string();
        assert!(self.item_ids.get(&id_str).is_none());
        self.item_ids.insert(id_str, id);

        if let Some(calc) = &item.condition {
            for value in calc.values() {
                self.value_mut(value).conditions.push(id);
            }
        }

        self.items.push(item);
        id
    }

    /// Value of `from` will be added to `to` with the given factor.
    pub fn add_dependency(&mut self, id: ValueId, calc: Calculation) {
        // TODO: prevent cycles
        for dependency in calc.values() {
            self.value_mut(dependency).dependents.push(id);
        }

        self.value_mut(id).dependencies.push(calc);
    }

    /// Get the type for a characters main inventory.
    pub fn main_inventory(&self) -> Option<InventoryId> {
        self.main_inventory
    }

    /// Set the main inventory for a character.
    pub fn set_main_inventory(&mut self, id: InventoryId) {
        self.main_inventory = Some(id)
    }

    /// When item `from` is equipped, `to` will be modified accordingly.
    pub fn add_modification(&mut self, from: ItemId, to: ValueId, modification: Modification) {
        // TODO: prevent cycles
        self.item_mut(from).modifications.insert(to, modification);
        self.value_mut(to).modifying_items.push(from);
    }

    /// Get the ValueId corresponding to an id string.
    pub fn value_id(&self, id: &str) -> ValueId {
        self.value_ids[id]
    }

    /// Get the InventoryId corresponding to an id string.
    pub fn inventory_id(&self, id: &str) -> InventoryId {
        self.inventory_ids[id]
    }

    /// Get the ItemId corresponding to an id string.
    pub fn item_id(&self, id: &str) -> ItemId {
        self.item_ids[id]
    }

    pub(crate) fn value(&self, id: ValueId) -> &Value {
        &self.values[id.idx()]
    }

    pub(crate) fn value_mut(&mut self, id: ValueId) -> &mut Value {
        &mut self.values[id.idx()]
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = (ValueId, &Value)> {
        self.values
            .iter()
            .enumerate()
            .map(|(id, val)| (ValueId(id as u32), val))
    }

    pub(crate) fn inventory(&self, id: InventoryId) -> &Inventory {
        &self.inventories[id.idx()]
    }

    pub(crate) fn item(&self, id: ItemId) -> &Item {
        &self.items[id.idx()]
    }

    pub(crate) fn item_mut(&mut self, id: ItemId) -> &mut Item {
        &mut self.items[id.idx()]
    }

    pub(crate) fn items(&self) -> impl Iterator<Item = (ItemId, &Item)> {
        self.items
            .iter()
            .enumerate()
            .map(|(id, val)| (ItemId(id as u32), val))
    }
}
