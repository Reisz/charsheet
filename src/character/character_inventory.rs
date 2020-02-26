use super::CharacterItem;
use crate::model::{InventoryId, ItemId, Physical};
use std::{cmp::min, convert::TryFrom};

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

    /// Clamp `amount` to the maximum amount of `item` that will still fit.
    fn limit_fill(&self, physical: &Physical, amount: u16, capacity: Option<u32>) -> u16 {
        if let Some(capacity) = capacity {
            let capacity = u16::try_from(capacity - self.fill).unwrap_or(std::u16::MAX);
            min(amount, capacity / physical.size)
        } else {
            amount
        }
    }

    /// Attempt to fill non-full stacks with àmount` of `item`. Returns number of items left.
    fn fill_stacks(&mut self, id: ItemId, mut amount: u16, limit: u16) -> u16 {
        for (slot_id, existing) in &mut self.content {
            if *slot_id == id {
                let space = limit - existing.count();
                let usage = min(space, amount);

                *existing.count_mut() += usage;
                amount -= usage;
            }
        }

        amount
    }

    /// Attempt to create new stacks with `amount` of `item`. Returns number of items left.
    fn create_stacks(
        &mut self,
        id: ItemId,
        mut amount: u16,
        limit: u16,
        slot_count: Option<usize>,
    ) -> u16 {
        while amount > 0 {
            if slot_count.map_or(false, |slots| self.content.len() == slots) {
                break;
            }

            let usage = min(limit, amount);

            self.content.push((id, CharacterItem::with_count(usage)));
            amount -= usage;
        }

        amount
    }

    /// Attempt to put `amount` of `item` into this inventory. Returns number of items that could not fit.
    pub(crate) fn put(
        &mut self,
        id: ItemId,
        physical: &Physical,
        amount: u16,
        capacity: Option<u32>,
        slot_count: Option<usize>,
    ) -> u16 {
        let to_put = self.limit_fill(physical, amount, capacity);
        let remainder = amount - to_put;

        let stack_size: u16 = physical.stack_size.into();

        let to_put = self.fill_stacks(id, to_put, stack_size);
        let to_put = self.create_stacks(id, to_put, stack_size, slot_count);

        let remainder = remainder + to_put;
        self.fill += u32::from(amount - remainder) * u32::from(physical.size);
        remainder
    }
}
