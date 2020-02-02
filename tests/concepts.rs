use charsheet::model::*;
use charsheet::Character;

#[test]
fn character_points() {
    let mut model = Model::new();

    let character_points = model.add_value(
        "character_points",
        Value::new(FrontEnd::new("Character Points"), 40),
    );
    let strength = model.add_value("strength", Value::new(FrontEnd::new("Strength"), 5));
    let min_strength = model.add_value(
        "min_strength",
        Value::new(FrontEnd::new("Minimum Strength"), 3),
    );
    let max_strength = model.add_value(
        "max_strength",
        Value::new(FrontEnd::new("Maximum Strength"), 12),
    );

    model.add_dependency(character_points, {
        let mut calc = Calculation::new();

        let min = calc.value(min_strength);
        let actual = calc.value(strength);
        let max = calc.value(max_strength);
        let out = calc.sub(min, actual);

        let extra = {
            let threshold = calc.constant(2);
            let threshold = calc.sub(max, threshold);
            let threshold = calc.sub(actual, threshold);

            let zero = calc.constant(0);
            calc.max(zero, threshold)
        };
        let out = calc.sub(out, extra);

        calc.set_output(out);
        calc
    });

    let mut char = Character::new(&model);

    assert_eq!(char.get(character_points), 38);
    char.set_base(strength, 10);
    assert_eq!(char.get(character_points), 33);
    char.set_base(strength, 11);
    assert_eq!(char.get(character_points), 31);
    char.set_base(strength, 12);
    assert_eq!(char.get(character_points), 29);
}
