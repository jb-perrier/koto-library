use koto::{parser::KString, runtime::{KMap, KNumber, KValue}};
use slab::Slab;

use crate::ValueId;

#[derive(Default)]
pub struct ModuleBuilder {
    pub values: Slab<KValue>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_str(&mut self, value: &str) -> ValueId {
        self.values.insert(KValue::Str(KString::from(value))) as ValueId
    }
    pub fn create_number(&mut self, value: f64) -> ValueId {
        self.values.insert(KValue::Number(KNumber::F64(value))) as ValueId
    }

    pub fn create_map(&mut self) -> ValueId {
        self.values.insert(KValue::Map(KMap::default())) as ValueId
    }

    pub fn map_insert(&mut self, map_id: ValueId, key: &str, value_id: ValueId) -> Result<(), ()> {
        // Get the value to insert
        let Some(value) = self.values.try_remove(value_id as usize) else {
            return Err(());
        };
        
        // Get mutable reference to the map and insert the value
        if let Some(KValue::Map(map)) = self.values.get_mut(map_id as usize) {
            map.insert(key, value);
            Ok(())
        } else {
            // Put the value back if map access failed
            self.values.insert(value);
            Err(())
        }
    }

    pub fn take_value(&mut self, id: ValueId) -> Option<KValue> {
        self.values.try_remove(id as usize)
    }
}