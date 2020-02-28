use charsheet::model::{Model, Value};
use charsheet::Character;

fn dnd_model() -> Model {
    let mut model = Model::new();

    // Abilities
    for &ability in [
        "strength",
        "dexterity",
        "constitution",
        "intelligence",
        "wisdom",
        "charisma",
    ]
    .iter()
    {
        let value = model.add_value(ability, Value::new(10));
        let modifier = model.add_value(ability.to_owned() + "_mod", Value::new(0));

        // Formula for modifier, derived from table (division rounds towards zero)
        model.add_dependency(modifier, (value / 2) - 5);
    }

    // TODO extend

    model
}

#[test]
fn test_ability_modifiers() {
    let model = dnd_model();

    let mut character = Character::new(&model);
    character.set_base(model.value_id("strength"), 1);
    character.set_base(model.value_id("dexterity"), 30);
    character.set_base(model.value_id("constitution"), 10);
    character.set_base(model.value_id("intelligence"), 11);

    assert_eq!(character.get(model.value_id("strength_mod")), -5);
    assert_eq!(character.get(model.value_id("dexterity_mod")), 10);
    assert_eq!(character.get(model.value_id("constitution_mod")), 0);
    assert_eq!(character.get(model.value_id("intelligence_mod")), 0);
}
