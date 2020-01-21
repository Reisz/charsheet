//! Allows defining rules for values and items.

use std::collections::HashMap;

macro_rules! id {
    ($name:ident) => {
        /// Wrapper to prevent mixing ids
        #[derive(Clone, Copy, PartialEq)]
        pub struct $name(u32);

        impl $name {
            pub(crate) fn idx(self) -> usize {
                self.0 as usize
            }
        }
    };
}

id!(ValueId);
id!(ItemId);

/// A value in the character sheet.
pub struct Value {
    base: i32,

    dependencies: Vec<(f32, ValueId)>,
    dependents: Vec<ValueId>,
}

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
    modifications: Vec<(ValueId, Modification)>,
}

/// Contains a set of values and items that can be used together.
pub struct Model {
    values: Vec<Value>,
    items: Vec<Item>,

    value_ids: HashMap<String, ValueId>,
    item_ids: HashMap<String, ItemId>,
}

/// Allows procedures in Model to manipulate values.
pub trait ValueStorage {
    /// Get a value.
    fn get(&self, id: ValueId) -> i32;
    /// Change a value
    fn set(&mut self, id: ValueId, value: i32);
}

impl Model {
    /// Add a new value to the model. Id string can not alias other value ids.
    pub fn add_value(&mut self, id_str: String, value: Value) -> ValueId {
        let id = ValueId(self.values.len() as u32);

        assert!(self.value_ids.get(&id_str).is_none());
        self.value_ids.insert(id_str, id);

        self.values.push(value);
        id
    }

    /// Add a new item to the model. Id string can not alias other item ids.
    pub fn add_item(&mut self, id_str: String, item: Item) -> ItemId {
        let id = ItemId(self.items.len() as u32);

        assert!(self.item_ids.get(&id_str).is_none());
        self.item_ids.insert(id_str, id);

        self.items.push(item);
        id
    }

    /// Get the ValueId corresponding to an id string.
    pub fn value_id(&self, id: &str) -> ValueId {
        self.value_ids[id]
    }

    /// Get the ItemId corresponding to an id string.
    pub fn item_id(&self, id: &str) -> ItemId {
        self.item_ids[id]
    }

    /// Update all values that depend on a changed value.
    pub fn update_dependents<S: ValueStorage>(&self, id: ValueId, storage: &mut S) {
        for dependent in &self.values[id.idx()].dependents {
            let mut value = self.values[dependent.idx()].base;

            for (factor, dependency) in &self.values[dependent.idx()].dependencies {
                value += (factor * storage.get(*dependency) as f32) as i32;
            }

            storage.set(*dependent, value);
        }
    }

    /// Apply new modifications to a value after an item has been equipped.
    pub fn apply_modifications<S: ValueStorage>(&self, item: ItemId, storage: &mut S) {
        for (id, modification) in &self.items[item.idx()].modifications {
            let value = storage.get(*id);
            storage.set(
                *id,
                match modification {
                    Modification::Add(summand) => value + summand,
                    Modification::Multiply(factor) => (value as f32 * factor) as i32,
                    Modification::Change(value) => *value,
                },
            );
        }
    }
}
