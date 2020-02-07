use charsheet::model::*;
use charsheet::Character;

#[test]
fn character_points() {
    let mut model = Model::new();

    let character_points = model.add_value("character_points", Value::new(40));
    let strength = model.add_value("strength", Value::new(5));
    let min_strength = model.add_value("min_strength", Value::new(3));
    let max_strength = model.add_value("max_strength", Value::new(12));

    model.add_dependency(
        character_points,
        min_strength - strength - Calculation::max(0.into(), strength - max_strength + 2),
    );

    let mut char = Character::new(&model);

    assert_eq!(char.get(character_points), 38);
    char.set_base(strength, 10);
    assert_eq!(char.get(character_points), 33);
    char.set_base(strength, 11);
    assert_eq!(char.get(character_points), 31);
    char.set_base(strength, 12);
    assert_eq!(char.get(character_points), 29);
}
