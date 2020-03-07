use super::{Calculation, Choice, FrontEnd, Id, Item};

/// A value in the character sheet.
pub struct Value {
    /// Front end data
    pub front_end: Option<FrontEnd>,

    pub(crate) default: i32,

    pub(crate) dependencies: Vec<Calculation>,
    pub(crate) modifying_items: Vec<Id<Item>>,
    pub(crate) modifying_choices: Vec<Id<Choice>>,
    pub(crate) dependents: Vec<Id<Value>>,
    pub(crate) conditions: Vec<Id<Item>>,
}

impl Value {
    /// Create a new value.
    pub fn new(default: i32) -> Self {
        Self {
            front_end: None,
            default,

            dependencies: Vec::new(),
            modifying_items: Vec::new(),
            modifying_choices: Vec::new(),
            dependents: Vec::new(),
            conditions: Vec::new(),
        }
    }

    /// Allow this item to be front-end visible.
    pub fn frontend(mut self, front_end: FrontEnd) -> Self {
        self.front_end = Some(front_end);
        self
    }
}
