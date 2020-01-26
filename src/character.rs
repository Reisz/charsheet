use crate::model::{ItemId, Model, Modification, ValueId};

struct CharacterValue {
    base: i32,
    actual: i32,
}

impl CharacterValue {
    fn new(base: i32) -> Self {
        Self { base, actual: base }
    }
}

/// Contains actual values and equipped items.
pub struct Character<'a> {
    model: &'a Model,
    values: Vec<CharacterValue>,
    items: Vec<u16>,
}

impl Character<'_> {
    /// Create a new character with a model.
    pub fn new(model: &'_ Model) -> Character<'_> {
        Character {
            model,
            values: model
                .values
                .iter()
                .map(|v| CharacterValue::new(v.default))
                .collect(),
            items: Vec::new(),
        }
    }

    fn value(&self, id: ValueId) -> &CharacterValue {
        &self.values[id.idx()]
    }

    fn value_mut(&mut self, id: ValueId) -> &mut CharacterValue {
        &mut self.values[id.idx()]
    }

    /// Get a value
    pub fn get(&self, id: ValueId) -> i32 {
        self.value(id).actual
    }

    /// Change a base value
    pub fn set_base(&mut self, id: ValueId, new: i32) {
        let value = self.value_mut(id);

        let old = value.base;
        if new == old {
            return;
        }

        value.base = new;
        self.update_value(id);
    }

    /// Add an item to the character.
    pub fn equip(&mut self, id: ItemId) {
        self.items[id.idx()] += 1;

        for value in self.model.item(id).modifications.keys() {
            self.update_value(*value);
        }
    }

    fn update_value(&mut self, id: ValueId) {
        let mut actual = self.value(id).base;

        for (factor, dependency) in &self.model.value(id).dependencies {
            actual += (factor * self.get(*dependency) as f32) as i32;
        }

        self.value_mut(id).actual = actual;
        self.apply_modifications(id);

        for dependent in &self.model.value(id).dependents {
            self.update_value(*dependent);
        }

        // TODO: update observers
        // NOTE: group values, only make groups observable
    }

    fn apply_modifications(&mut self, id: ValueId) {
        let mods: Vec<_> = self
            .model
            .value(id)
            .modifying_items
            .iter()
            .map(|item| &self.model.item(*item).modifications[&id])
            .collect();

        // TODO: determine sorting

        let mut value = self.get(id);
        for modification in mods {
            value = match modification {
                Modification::Add(summand) => value + summand,
                Modification::Multiply(factor) => (value as f32 * factor) as i32,
                Modification::Change(value) => *value,
            };
        }
        self.value_mut(id).actual = value;
    }
}
