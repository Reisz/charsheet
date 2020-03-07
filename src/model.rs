//! Allows defining rules for values and items.

mod calculation;
mod choice;
mod container;
mod front_end;
mod inventory;
mod item;
mod modification;
mod value;

pub use calculation::*;
pub use choice::*;
pub use container::*;
pub use front_end::*;
pub use inventory::*;
pub use item::*;
pub use modification::*;
pub use value::*;

/// Contains a set of values and items that can be used together.
#[derive(Default)]
pub struct Model {
    choices: Container<Choice>,
    values: Container<Value>,
    inventories: Container<Inventory>,
    items: Container<Item>,

    main_inventory: Option<Id<Inventory>>,
}

impl Model {
    /// Create a new Model.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new choice to the Model. Id string can not alias other choice ids.
    pub fn add_choice(&mut self, id_str: impl ToString, choice: Choice) -> Id<Choice> {
        self.choices.insert(id_str, choice)
    }

    /// Add a new value to the model. Id string can not alias other value ids.
    pub fn add_value(&mut self, id_str: impl ToString, value: Value) -> Id<Value> {
        self.values.insert(id_str, value)
    }

    /// Add a new inventory type.
    pub fn add_inventory(&mut self, id_str: impl ToString, inventory: Inventory) -> Id<Inventory> {
        self.inventories.insert(id_str, inventory)
    }

    /// Add a new item to the model. Id string can not alias other item ids.
    pub fn add_item(&mut self, id_str: impl ToString, item: Item) -> Id<Item> {
        let id = self.items.insert(id_str, item);

        if let Some(calc) = &self.items.get(id).condition {
            for value in calc.values() {
                self.values.get_mut(value).conditions.push(id);
            }
        }

        id
    }

    /// Value of `from` will be added to `to` with the given factor.
    pub fn add_dependency(&mut self, id: Id<Value>, calc: impl IntoCalculation) {
        let calc = calc.into_calc();

        // TODO: prevent cycles
        for dependency in calc.values() {
            self.values.get_mut(dependency).dependents.push(id);
        }

        self.values.get_mut(id).dependencies.push(calc);
    }

    /// Add a selection to a choice.
    pub fn add_selection(&mut self, id: Id<Choice>, selection: Selection) {
        for &value in selection.modifications.keys() {
            let list = &mut self.values.get_mut(value).modifying_choices;
            if list.iter().all(|&e| e != id) {
                list.push(id);
            }
        }

        self.choices.get_mut(id).options.push(selection);
    }

    /// Get the type for a characters main inventory.
    pub fn main_inventory(&self) -> Option<Id<Inventory>> {
        self.main_inventory
    }

    /// Set the main inventory for a character.
    pub fn set_main_inventory(&mut self, id: Id<Inventory>) {
        self.main_inventory = Some(id)
    }

    /// When item `from` is equipped, `to` will be modified accordingly.
    pub fn add_modification(
        &mut self,
        from: Id<Item>,
        to: Id<Value>,
        mut modification: Modification,
    ) {
        // TODO: prevent cycles
        modification.set_value(to);
        self.items
            .get_mut(from)
            .modifications
            .insert(to, modification);
        self.values.get_mut(to).modifying_items.push(from);
    }

    /// Returns a reference to the Container of Choices.
    pub fn choices(&self) -> &Container<Choice> {
        &self.choices
    }

    /// Returns a reference to the Container of Values.
    pub fn values(&self) -> &Container<Value> {
        &self.values
    }

    /// Returns a reference to the Container of Inventories.
    pub fn inventories(&self) -> &Container<Inventory> {
        &self.inventories
    }

    /// Returns a reference to the Container of Items.
    pub fn items(&self) -> &Container<Item> {
        &self.items
    }
}
