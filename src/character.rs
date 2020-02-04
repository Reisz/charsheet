use crate::model::{
    Calculation, Inventory, InventoryId, InventoryInfo, ItemId, Model, Modification, ValueId,
};
use std::collections::HashSet;
use std::{
    cmp::min,
    convert::{TryFrom, TryInto},
};

struct CharacterValue {
    base: i32,
    actual: i32,
}

impl CharacterValue {
    fn new(base: i32) -> Self {
        Self { base, actual: base }
    }
}

struct CharacterInventory {
    inventory: InventoryId,
    content: Vec<(ItemId, u16)>,
    fill: u32,
}

impl CharacterInventory {
    fn new(inventory: InventoryId) -> Self {
        Self {
            inventory,
            content: Vec::new(),
            fill: 0,
        }
    }
}

/// Contains actual values and equipped items.
pub struct Character<'a> {
    model: &'a Model,
    values: Vec<CharacterValue>,
    inventories: Vec<CharacterInventory>,
    items: Vec<u16>,
}

impl Character<'_> {
    /// Create a new character with a model.
    pub fn new(model: &'_ Model) -> Character<'_> {
        let mut result = Character {
            model,
            values: model
                .values()
                .map(|(_, v)| CharacterValue::new(v.default))
                .collect(),
            inventories: model
                .main_inventory()
                .into_iter()
                .map(CharacterInventory::new)
                .collect(),
            items: model.items().map(|_| 0).collect(),
        };

        result.update_all_values();
        result
    }

    fn update_all_values(&mut self) {
        let mut todo: Vec<_> = self.model.values().collect();
        let mut done = HashSet::new();

        while let Some((id, value)) = todo.pop() {
            let ok = if value.dependencies.is_empty() {
                true
            } else if value
                .dependencies
                .iter()
                .map(|dep| dep.values())
                .flatten()
                .all(|dep| done.contains(&dep))
            {
                self.apply_dependencies(id);
                true
            } else {
                false
            };

            if ok {
                done.insert(id);
                for dependent in &value.dependents {
                    todo.push((*dependent, self.model.value(*dependent)));
                }
            }
        }
    }

    fn value(&self, id: ValueId) -> &CharacterValue {
        &self.values[id.idx()]
    }

    fn value_mut(&mut self, id: ValueId) -> &mut CharacterValue {
        &mut self.values[id.idx()]
    }

    fn inventory_idx(&self, item: Option<ItemId>) -> usize {
        item.map_or(0, |_| todo!())
    }

    fn item(&self, id: ItemId) -> u16 {
        self.items[id.idx()]
    }

    fn item_mut(&mut self, id: ItemId) -> &mut u16 {
        &mut self.items[id.idx()]
    }

    fn eval(&self, calc: &Calculation) -> i32 {
        calc.get(&calc.values().map(|id| self.get(id)).collect::<Vec<_>>())
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

    /// Store an item into an inventory. Returns the amount that could not fit.
    pub fn store(&mut self, inventory: Option<ItemId>, item: ItemId, to_put: u16) -> u16 {
        let inventory = self.inventory_idx(inventory);

        // Get item info & check size against current fill
        let InventoryInfo { size, stack_size } =
            self.model.item(item).inventory_info.as_ref().unwrap();
        let Inventory { capacity, slots } =
            &self.model.inventory(self.inventories[inventory].inventory);

        // Limit to_put to available space & calculate remainder
        let (mut to_put, remainder) = if let Some(capacity) = capacity {
            let space =
                u32::try_from(self.eval(capacity)).unwrap() - self.inventories[inventory].fill;
            let usage = min(space.try_into().unwrap_or(std::u16::MAX) / *size, to_put);

            (usage, to_put - usage)
        } else {
            (to_put, 0)
        };

        let fill = to_put;

        // Fill up exisitng stacks
        for (id, existing) in &mut self.inventories[inventory].content {
            if *id == item {
                let space = u16::from(*stack_size) - *existing;
                let usage = min(space, to_put);

                *existing += usage;
                to_put -= usage;
            }
        }

        // Calulcate exact amount of available slots
        let slots = slots
            .as_ref()
            .map(|slots| self.eval(slots).try_into().unwrap());

        // Create new stacks for remaining items.
        let inventory = &mut self.inventories[inventory];
        while to_put > 0 {
            if slots.map_or(false, |slots| inventory.content.len() == slots) {
                break;
            }

            let usage = min((*stack_size).into(), to_put);

            inventory.content.push((item, usage));
            to_put -= usage;
        }

        // update current fill
        inventory.fill += u32::from(fill - to_put) * u32::from(*size);

        // return remaining items
        to_put + remainder
    }

    /// Add an item to the character.
    pub fn equip(&mut self, id: ItemId) {
        self.items[id.idx()] += 1;

        for value in self.model.item(id).modifications.keys() {
            self.update_value(*value);
        }
    }

    fn apply_dependencies(&mut self, id: ValueId) {
        let mut actual = self.value(id).base;

        for calc in &self.model.value(id).dependencies {
            actual += calc.get(&calc.values().map(|id| self.get(id)).collect::<Vec<_>>());
        }

        self.value_mut(id).actual = actual;
    }

    fn update_value(&mut self, id: ValueId) {
        self.apply_dependencies(id);
        self.apply_modifications(id);

        for dependent in &self.model.value(id).dependents {
            self.update_value(*dependent);
        }

        for condition in &self.model.value(id).conditions {
            self.update_condition(*condition);
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
            .filter_map(|&item| {
                let count = self.item(item);

                if count > 0 {
                    Some((count, &self.model.item(item).modifications[&id]))
                } else {
                    None
                }
            })
            .collect();

        // TODO: determine sorting

        let mut value = self.get(id);
        for modification in mods {
            let (count, modification) = modification;

            value = match modification {
                Modification::Add(summand) => value + (summand * count as i32),
                Modification::Multiply(factor) => (value as f32 * factor.powf(count.into())) as i32,
                Modification::Change(value) => *value,
            };
        }
        self.value_mut(id).actual = value;
    }

    fn update_condition(&mut self, id: ItemId) {
        *self.item_mut(id) = if let Some(calc) = &self.model.item(id).condition {
            self.eval(calc) as u16
        } else {
            unreachable!();
        };

        for value in self.model.item(id).modifications.keys() {
            self.update_value(*value);
        }
    }
}
