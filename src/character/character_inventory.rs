use super::CharacterItem;
use crate::model::{InventoryId, ItemId};

pub struct CharacterInventory {
    id: InventoryId,
    pub content: Vec<(ItemId, CharacterItem)>,
    pub fill: u32,
}

impl CharacterInventory {
    pub fn new(id: InventoryId) -> Self {
        Self {
            id,
            content: Vec::new(),
            fill: 0,
        }
    }

    pub fn id(&self) -> InventoryId {
        self.id
    }
}
