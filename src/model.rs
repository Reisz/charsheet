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

/// Stores front end data.
pub struct FrontEnd {
    /// Name of the element
    pub name: String,
    /// Short name of the element
    pub name_short: Option<String>,
    /// Description of the element
    pub description: Option<String>,
}

impl FrontEnd {
    /// Create a new set of front end values.
    pub fn new(name: String) -> Self {
        FrontEnd {
            name,
            name_short: None,
            description: None,
        }
    }
}

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

    pub(crate) modifications: Vec<(ValueId, Modification)>,
}

impl Item {
    /// Create a new item.
    pub fn new(front_end: FrontEnd) -> Self {
        Self {
            front_end,

            modifications: Vec::new(),
        }
    }
}

/// Contains a set of values and items that can be used together.
pub struct Model {
    pub(crate) values: Vec<Value>,
    pub(crate) items: Vec<Item>,

    value_ids: HashMap<String, ValueId>,
    item_ids: HashMap<String, ItemId>,
}

impl Model {
    /// Create a new Model.
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            items: Vec::new(),

            value_ids: HashMap::new(),
            item_ids: HashMap::new(),
        }
    }

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
}
