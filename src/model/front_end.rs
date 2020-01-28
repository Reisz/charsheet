/// Stores front end data.
pub struct FrontEnd {
    /// Name of the element
    pub name: String,
    /// Short name of the element
    pub name_short: Option<String>,
    /// Description of the element
    pub description: Option<String>,
}

impl FrontEnd {
    /// Create a new set of front end values.
    pub fn new(name: impl ToString) -> Self {
        FrontEnd {
            name: name.to_string(),
            name_short: None,
            description: None,
        }
    }
}
