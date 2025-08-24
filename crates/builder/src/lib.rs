use std::{
    ffi::{CStr, c_char},
    os::raw::c_void,
};

use koto::{
    parser::KString,
    runtime::{KMap, KNumber, KValue},
};
use slab::Slab;

pub type NativeFunction = extern "C" fn(args: ValueId) -> ValueId;
pub type ValueId = isize;
// pub type Map = usize;
// pub type Str = usize;
// pub type Number = usize;
// pub type NativeFunction = usize;

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn koto_create_str(module: *mut c_void, value: *const c_char) -> ValueId {
    if module.is_null() || value.is_null() {
        return -1;
    }

    let module = unsafe { &mut *(module as *mut ModuleBuilder) };
    let c_str = unsafe { CStr::from_ptr(value) };
    match c_str.to_str() {
        Ok(str_val) => module.create_str(str_val),
        Err(_) => -1,
    }
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn koto_create_number(module: *mut c_void, value: f64) -> ValueId {
    // Safety checks
    if module.is_null() {
        return -1;
    }

    let module = unsafe { &mut *(module as *mut ModuleBuilder) };
    module.create_number(value)
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn koto_create_map(module: *mut c_void) -> ValueId {
    if module.is_null() {
        return -1;
    }

    let module = unsafe { &mut *(module as *mut ModuleBuilder) };
    module.create_map()
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn koto_map_insert(
    module: *mut c_void,
    map: ValueId,
    key: *const c_char,
    value: ValueId,
) -> bool {
    // Safety checks
    if module.is_null() || key.is_null() {
        return false;
    }

    let module = unsafe { &mut *(module as *mut ModuleBuilder) };
    let c_str = unsafe { CStr::from_ptr(key) };

    match c_str.to_str() {
        Ok(key_str) => module.map_insert(map, key_str, value),
        Err(_) => false, // Invalid UTF-8 in key
    }
}

// fn koto_create_native_function(function: NativeFunction) -> ValueId;

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

    pub fn map_insert(&mut self, map_id: ValueId, key: &str, value_id: ValueId) -> bool {
        // Get the value to insert
        let Some(value) = self.values.try_remove(value_id as usize) else {
            return false;
        };
        
        // Get mutable reference to the map and insert the value
        if let Some(KValue::Map(map)) = self.values.get_mut(map_id as usize) {
            map.insert(key, value);
            true
        } else {
            // Put the value back if map access failed
            self.values.insert(value);
            false
        }
    }

    pub fn take_value(&mut self, id: ValueId) -> Option<KValue> {
        self.values.try_remove(id as usize)
    }
}
