//! Allows defining rules for values and items.

mod front_end;
mod item;
mod value;

pub use front_end::*;
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
id!(ItemId);

/// Contains a set of values and items that can be used together.
#[derive(Default)]
pub struct Model {
    pub(crate) values: Vec<Value>,
    pub(crate) items: Vec<Item>,

    value_ids: HashMap<String, ValueId>,
    item_ids: HashMap<String, ItemId>,
}

impl Model {
    /// Create a new Model.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new value to the model. Id string can not alias other value ids.
    pub fn add_value<S: ToString>(&mut self, id_str: S, value: Value) -> ValueId {
        let id = ValueId(self.values.len() as u32);

        let id_str = id_str.to_string();
        assert!(self.value_ids.get(&id_str).is_none());
        self.value_ids.insert(id_str, id);

        self.values.push(value);
        id
    }

    /// Add a new item to the model. Id string can not alias other item ids.
    pub fn add_item<S: ToString>(&mut self, id_str: S, item: Item) -> ItemId {
        let id = ItemId(self.items.len() as u32);

        let id_str = id_str.to_string();
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

    pub(crate) fn value(&self, id: ValueId) -> &Value {
        &self.values[id.idx()]
    }

    pub(crate) fn item(&self, id: ItemId) -> &Item {
        &self.items[id.idx()]
    }
}
