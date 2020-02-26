use std::collections::HashMap;
use super::{ValueId, Modification, FrontEnd};

pub struct Selection {
    pub(crate) front_end: Option<FrontEnd>,
    pub(crate) modifications: HashMap<ValueId, Modification>,
}

pub struct Choice {
    pub(crate) front_end: Option<FrontEnd>,
    pub(crate) options: Vec<Selection>,
}

impl Selection {
    
}

impl Choice {

}
