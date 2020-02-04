pub struct CharacterValue {
    pub base: i32,
    pub actual: i32,
}

impl CharacterValue {
    pub fn new(base: i32) -> Self {
        Self { base, actual: base }
    }
}
