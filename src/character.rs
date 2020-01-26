use crate::model::{ItemId, Model, Modification, ValueId};

/// Contains actual values and equipped items.
pub struct Character<'a> {
    model: &'a Model,
    values: Vec<i32>,
    items: Vec<u16>,
}

impl Character<'_> {
    /// Create a new character with a model.
    pub fn new(model: &'_ Model) -> Character<'_> {
        Character {
            model,
            values: Vec::new(),
            items: Vec::new(),
        }
    }

    /// Get a value
    pub fn get(&self, id: ValueId) -> i32 {
        self.values[id.idx()]
    }

    /// Change a value
    pub fn set(&mut self, id: ValueId, value: i32) {
        let old = self.values[id.idx()];
        if value == old {
            return;
        }

        self.values[id.idx()] = value;
        self.update_dependents(id);

        // TODO: update observers
        // NOTE: group values, only make groups observable
    }

    /// Add an item to the character.
    pub fn equip(&mut self, id: ItemId) {
        self.items[id.idx()] += 1;
        self.apply_modifications(id);
    }

    fn update_dependents(&mut self, id: ValueId) {
        for dependent in &self.model.values[id.idx()].dependents {
            let mut value = self.model.values[dependent.idx()].base;

            for (factor, dependency) in &self.model.values[dependent.idx()].dependencies {
                value += (factor * self.get(*dependency) as f32) as i32;
            }

            self.set(*dependent, value);
        }
    }

    fn apply_modifications(&mut self, item: ItemId) {
        for (id, modification) in &self.model.items[item.idx()].modifications {
            let value = self.get(*id);
            self.set(
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
