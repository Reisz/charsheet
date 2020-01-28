use super::{FrontEnd, ValueId};
use std::collections::HashMap;

/// Represents all the ways a Value can be modified by an Item.
pub enum Modification {
    /// Add to or subtract from a value
    Add(i32),
    /// Multiply a value
    Multiply(f32),
    /// Change to a predefined value
    Change(i32),
}

pub enum ConditionInput {
    Const(i32),
    Value(f32, ValueId),
}

pub enum ConditionOperator {
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt,
}

pub struct Condition {
    pub(crate) op: ConditionOperator,
    pub(crate) a: ConditionInput,
    pub(crate) b: ConditionInput,
}

impl Condition {
    pub fn new(a: ConditionInput, op: ConditionOperator, b: ConditionInput) -> Self {
        Self { a, op, b }
    }
}

/// "Equippable" item. Can be used to represent actual items, learnable skills, traits or other
/// conditionals.
pub struct Item {
    /// Front end data
    pub front_end: FrontEnd,

    pub(crate) condition: Option<Condition>,
    pub(crate) modifications: HashMap<ValueId, Modification>,
}

impl Item {
    /// Create a new item.
    pub fn new(front_end: FrontEnd) -> Self {
        Self {
            front_end,

            condition: None,
            modifications: HashMap::new(),
        }
    }

    pub fn with_condition(front_end: FrontEnd, condition: Condition) -> Self {
        Self {
            front_end,

            condition: Some(condition),
            modifications: HashMap::new(),
        }
    }
}
