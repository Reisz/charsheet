mod character_inventory;
mod character_item;
mod character_value;

use self::character_inventory::*;
use self::character_item::*;
use self::character_value::*;

use crate::model::{Calculation, Choice, Id, Inventory, Item, Model, Value};
use std::collections::HashSet;
use std::convert::TryFrom;

/// Points to the inventory of an item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemInventory(usize);

/// Contains actual values and equipped items.
pub struct Character<'a> {
    model: &'a Model,
    choices: Vec<u16>,
    inventories: Vec<CharacterInventory>,
    items: Vec<CharacterItem>,
    values: Vec<CharacterValue>,
}

impl Character<'_> {
    /// Create a new character with a model.
    pub fn new(model: &'_ Model) -> Character<'_> {
        let mut result = Character {
            model,
            choices: model.choices().iter().map(|_| 0).collect(),
            values: model
                .values()
                .iter()
                .map(|(_, v)| CharacterValue::new(v.default))
                .collect(),
            inventories: model
                .main_inventory()
                .into_iter()
                .map(CharacterInventory::new)
                .collect(),
            items: model
                .items()
                .iter()
                .map(|(_, item)| CharacterItem::new(item.has_inventory))
                .collect(),
        };

        result.update_all_values();
        result
    }

    fn update_all_values(&mut self) {
        let mut todo: Vec<_> = self.model.values().iter().collect();
        let mut done = HashSet::new();

        while let Some((id, value)) = todo.pop() {
            let ok = if value.dependencies.is_empty() {
                self.apply_modifications(id);
                true
            } else if value
                .dependencies
                .iter()
                .map(|dep| dep.values())
                .flatten()
                .all(|dep| done.contains(&dep))
            {
                self.apply_dependencies(id);
                self.apply_modifications(id);
                true
            } else {
                false
            };

            if ok {
                done.insert(id);
                for dependent in &value.dependents {
                    todo.push((*dependent, self.model.values().get(*dependent)));
                }
            }
        }
    }

    fn choice(&self, id: Id<Choice>) -> u16 {
        self.choices[id.0]
    }

    fn item(&self, id: Id<Item>) -> &CharacterItem {
        &self.items[id.0]
    }

    fn item_mut(&mut self, id: Id<Item>) -> &mut CharacterItem {
        &mut self.items[id.0]
    }

    fn value(&self, id: Id<Value>) -> &CharacterValue {
        &self.values[id.0]
    }

    fn value_mut(&mut self, id: Id<Value>) -> &mut CharacterValue {
        &mut self.values[id.0]
    }

    fn eval(&self, calc: &Calculation) -> i32 {
        calc.get(&calc.values().map(|id| self.get(id)).collect::<Vec<_>>())
    }

    /// Get a value
    pub fn get(&self, id: Id<Value>) -> i32 {
        self.value(id).actual
    }

    /// Change a base value
    pub fn set_base(&mut self, id: Id<Value>, new: i32) {
        let value = self.value_mut(id);

        let old = value.base;
        if new == old {
            return;
        }

        value.base = new;
        self.update_value(id);
    }

    /// Store an item into an inventory. Returns the amount that could not fit.
    pub fn store(&mut self, inventory: Option<Id<Inventory>>, item: Id<Item>, amount: u16) -> u16 {
        let inventory = inventory.unwrap_or_else(|| Id::new(0)).0;

        let Inventory { capacity, slots } = &self
            .model
            .inventories()
            .get(self.inventories[inventory].id());

        let capacity = capacity
            .as_ref()
            .map(|capacity| u32::try_from(self.eval(capacity)).unwrap());
        let slots = slots
            .as_ref()
            .map(|slots| usize::try_from(self.eval(slots)).unwrap());
        let physical = self.model.items().get(item).physical.as_ref().unwrap();
        self.inventories[inventory].put(item, physical, amount, capacity, slots)
    }

    /// Add an item to the character.
    pub fn equip(&mut self, id: Id<Item>) {
        *self.items[id.0].count_mut() += 1;

        for value in self.model.items().get(id).modifications.keys() {
            self.update_value(*value);
        }
    }

    fn eval_calc(&self, calc: &Calculation) -> i32 {
        calc.get(&calc.values().map(|id| self.get(id)).collect::<Vec<_>>())
    }

    fn apply_dependencies(&mut self, id: Id<Value>) {
        let mut actual = self.value(id).base;

        for calc in &self.model.values().get(id).dependencies {
            actual += self.eval_calc(calc);
        }

        self.value_mut(id).actual = actual;
    }

    fn update_value(&mut self, id: Id<Value>) {
        self.apply_dependencies(id);
        self.apply_modifications(id);

        for dependent in &self.model.values().get(id).dependents {
            self.update_value(*dependent);
        }

        for condition in &self.model.values().get(id).conditions {
            self.update_condition(*condition);
        }

        // TODO: update observers
        // NOTE: group values, only make groups observable
    }

    fn apply_modifications(&mut self, id: Id<Value>) {
        let mut mods: Vec<_> = self
            .model
            .values()
            .get(id)
            .modifying_items
            .iter()
            .filter_map(|&item| {
                let count = self.item(item).count();

                if count > 0 {
                    Some((count, &self.model.items().get(item).modifications[&id]))
                } else {
                    None
                }
            })
            .chain(
                self.model
                    .values()
                    .get(id)
                    .modifying_choices
                    .iter()
                    .filter_map(|&choice| {
                        self.model.choices().get(choice).options[self.choice(choice) as usize]
                            .modifications
                            .get(&id)
                            .map(|m| (1, m))
                    }),
            )
            .collect();

        // Sort by priority
        mods.sort_unstable_by(|(_, a), (_, b)| a.priority().cmp(&b.priority()));

        for modification in mods {
            let (count, modification) = modification;

            let calc = modification.calculation();
            for _ in 0..count {
                self.value_mut(id).actual = self.eval_calc(calc);
            }
        }
    }

    fn update_condition(&mut self, id: Id<Item>) {
        *self.item_mut(id).count_mut() = if let Some(calc) = &self.model.items().get(id).condition {
            self.eval(calc) as u16
        } else {
            unreachable!();
        };

        for value in self.model.items().get(id).modifications.keys() {
            self.update_value(*value);
        }
    }
}
