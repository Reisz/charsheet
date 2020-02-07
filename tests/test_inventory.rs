use charsheet::model::*;
use charsheet::Character;

#[test]
fn simple_item() {
    let mut model = Model::new();

    let inventory = model.add_inventory("main", Inventory::new());
    model.set_main_inventory(inventory);

    let chestplate = model.add_item("chestplate", Item::new().allow_in_inventory(10, 1));

    let mut character = Character::new(&model);

    assert_eq!(character.store(None, chestplate, 1), 0);
}

#[test]
fn limited_space() {
    let mut model = Model::new();

    let inventory = model.add_inventory("main", Inventory::new().capacity(15));
    model.set_main_inventory(inventory);

    let chestplate = model.add_item("chestplate", Item::new().allow_in_inventory(10, 1));

    let mut character = Character::new(&model);

    assert_eq!(character.store(None, chestplate, 2), 1);
}

#[test]
fn limited_slots() {
    let mut model = Model::new();

    let inventory = model.add_inventory("main", Inventory::new().slots(1));
    model.set_main_inventory(inventory);

    let chestplate = model.add_item("chestplate", Item::new().allow_in_inventory(10, 1));

    let mut character = Character::new(&model);

    assert_eq!(character.store(None, chestplate, 2), 1);
}

#[test]
fn limited_slots_space() {
    let mut model = Model::new();

    let inventory = model.add_inventory("main", Inventory::new().slots(2).capacity(15));
    model.set_main_inventory(inventory);

    let chestplate = model.add_item("chestplate", Item::new().allow_in_inventory(10, 1));
    let paper_sheet = model.add_item("paper_sheet", Item::new().allow_in_inventory(1, 10));

    let mut character = Character::new(&model);

    assert_eq!(character.store(None, chestplate, 1), 0);
    assert_eq!(character.store(None, paper_sheet, 10), 5);
    assert_eq!(character.store(None, chestplate, 1), 1);
}

#[test]
fn stacking() {
    let mut model = Model::new();

    let inventory = model.add_inventory("main", Inventory::new().slots(3));
    model.set_main_inventory(inventory);

    let chestplate = model.add_item("chestplate", Item::new().allow_in_inventory(10, 1));
    let paper_sheet = model.add_item("paper_sheet", Item::new().allow_in_inventory(1, 10));

    let mut character = Character::new(&model);

    assert_eq!(character.store(None, paper_sheet, 5), 0);
    assert_eq!(character.store(None, chestplate, 1), 0);
    assert_eq!(character.store(None, paper_sheet, 10), 0);
    assert_eq!(character.store(None, paper_sheet, 10), 5);
}
