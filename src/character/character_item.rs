use super::ItemInventory;
// use std::convert::TryFrom;
use crate::model::InventoryId;

pub struct CharacterItem {
    inventories: Option<Vec<ItemInventory>>,
    count: u16,
}

impl CharacterItem {
    pub fn new(has_inventory: Option<InventoryId>) -> Self {
        Self {
            inventories: has_inventory.map(|_| Vec::new()),
            count: 0,
        }
    }

    pub fn with_count(count: u16) -> Self {
        Self {
            inventories: None,
            count,
        }
    }

    // pub fn inventory(&self, idx: u16) -> Option<ItemInventory> {
    //     self.inventories.as_ref().map(|inventories| inventories[idx as usize])
    // }

    pub fn count(&self) -> u16 {
        self.count
    }

    pub fn count_mut(&mut self) -> &mut u16 {
        assert!(self.inventories.is_none());
        &mut self.count
    }

    // pub fn push(&mut self, item: ItemInventory) -> u16 {
    //     let inventories = self.inventories.as_mut().unwrap();
    //
    //     let idx = u16::try_from(inventories.len()).unwrap();
    //     inventories.push(item);
    //     self.count += 1;
    //
    //     idx
    // }
    //
    // pub fn remove(&mut self, idx: u16) -> ItemInventory {
    //     let inventories = self.inventories.as_mut().unwrap();
    //
    //     self.count -= 1;
    //     inventories.remove(idx as usize)
    // }
}
