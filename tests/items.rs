use charsheet::model::{Calculation, FrontEnd, Item, Model, Modification, Value};
use charsheet::Character;

#[test]
fn simple_modification() {
    let mut model = Model::new();

    let armor = model.add_value("armor", Value::new(FrontEnd::new("Armor"), 0));
    let chestplate = model.add_item("chestplate", Item::new(FrontEnd::new("Chestplate")));
    model.add_modification(chestplate, armor, Modification::Add(10));

    let mut character = Character::new(&model);

    assert_eq!(character.get(armor), 0);
    character.equip(chestplate);
    assert_eq!(character.get(armor), 10);
}

#[test]
fn conditional_item() {
    let mut model = Model::new();
    let initiative = model.add_value("initiative", Value::new(FrontEnd::new("Initiative"), 0));
    let burden = model.add_value("burden", Value::new(FrontEnd::new("Carrying"), 0));
    let max_burden = model.add_value(
        "max_burden",
        Value::new(FrontEnd::new("Carrying capacity"), 20),
    );

    let overburdened = model.add_item(
        "overburdened",
        Item::with_condition(
            FrontEnd::new("Overburdened"),
            Calculation::from(burden).gt(max_burden),
        ),
    );
    model.add_modification(overburdened, initiative, Modification::Add(-2));

    let mut character = Character::new(&model);

    assert_eq!(character.get(initiative), 0);
    character.set_base(burden, 20);
    assert_eq!(character.get(initiative), 0);
    character.set_base(burden, 21);
    assert_eq!(character.get(initiative), -2);
}
