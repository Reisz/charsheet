use crate::model::*;

/// Contains actual values and equipped items.
pub struct Character<'a> {
    model: &'a Model,
    values: Vec<i32>,
    items: Vec<u16>,
}

impl ValueStorage for Character<'_> {
    fn get(&self, id: ValueId) -> i32 {
        self.values[id.idx()]
    }

    fn set(&mut self, id: ValueId, value: i32) {
        let old = self.values[id.idx()];
        if value == old {
            return;
        }

        self.values[id.idx()] = value;
        self.model.update_dependents(id, self);

        // TODO: update observers
        // NOTE: group values, only make groups observable
    }
}

impl Character<'_> {
    /// Add an item to the character.
    pub fn equip(&mut self, id: ItemId) {
        self.items[id.idx()] += 1;
        self.model.apply_modifications(id, self);
    }
}
