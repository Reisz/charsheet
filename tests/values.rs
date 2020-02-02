use charsheet::model::{Calculation, FrontEnd, Model, Value};
use charsheet::Character;

#[test]
fn simple_value() {
    let mut model = Model::new();
    let strength = model.add_value("strength", Value::new(FrontEnd::new("Strength"), 0));

    let mut char = Character::new(&model);

    assert_eq!(char.get(strength), 0);

    char.set_base(strength, 2);
    assert_eq!(char.get(strength), 2);
}

#[test]
fn base_value() {
    let mut model = Model::new();
    let strength = model.add_value("strength", Value::new(FrontEnd::new("Strength"), 2));

    let char = Character::new(&model);

    assert_eq!(char.get(strength), 2);
}

#[test]
fn dependent_value() {
    let mut model = Model::new();
    let strength = model.add_value("strength", Value::new(FrontEnd::new("Strength"), 2));
    let max_burden = model.add_value(
        "max_burden",
        Value::new(FrontEnd::new("Carrying capacity"), 20),
    );
    model.add_dependency(max_burden, {
        let mut calc = Calculation::new();

        let c = calc.constant(10);
        let val = calc.value(strength);
        let mul = calc.multiply(c, val);

        calc.set_output(mul);
        calc
    });

    let mut char = Character::new(&model);

    assert_eq!(char.get(max_burden), 40);
    char.set_base(strength, 0);
    assert_eq!(char.get(max_burden), 20);
}

#[test]
fn multiple_dependencies() {
    let mut model = Model::new();
    let dexterity = model.add_value("dexterity", Value::new(FrontEnd::new("Dexterity"), 2));
    let perception = model.add_value("perception", Value::new(FrontEnd::new("Perception"), 1));

    let initiative = model.add_value("initiative", Value::new(FrontEnd::new("Initiative"), 0));
    model.add_dependency(initiative, {
        let mut calc = Calculation::new();

        let a = calc.value(perception);
        let b = calc.value(dexterity);
        let add = calc.add(a, b);

        calc.set_output(add);
        calc
    });

    let mut char = Character::new(&model);

    assert_eq!(char.get(initiative), 3);
    char.set_base(dexterity, 0);
    assert_eq!(char.get(initiative), 1);
    char.set_base(perception, 2);
    assert_eq!(char.get(initiative), 2);
}
