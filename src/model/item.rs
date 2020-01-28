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

/// Input value for an item condition.
pub enum ConditionInput {
    /// Compare against a constant
    Const(i32),
    /// Compare against a character value
    Value(f32, ValueId),
}

/// Comparison operator for coditions
pub enum ConditionOperator {
    /// Both sides have to be equal.
    Eq,
    /// Both sides can not be equal.
    Ne,
    /// A is less than or equal to B.
    Le,
    /// A is strictly less than B.
    Lt,
    /// A is more than or equal to B.
    Ge,
    /// A is strictly equal to B.
    Gt,
}

/// Condition controlling activation of an item.
pub struct Condition {
    pub(crate) op: ConditionOperator,
    pub(crate) a: ConditionInput,
    pub(crate) b: ConditionInput,
}

impl Condition {
    /// Create a new condition.
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

    /// Create a new item, which automatically applies itself based on the given condition.
    pub fn with_condition(front_end: FrontEnd, condition: Condition) -> Self {
        Self {
            front_end,

            condition: Some(condition),
            modifications: HashMap::new(),
        }
    }
}
