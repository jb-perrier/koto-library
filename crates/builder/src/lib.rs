#![allow(clippy::result_unit_err)]
use std::{
    ffi::{c_char, c_int, CStr},
    os::raw::c_void,
};
use koto::{
    parser::KString,
    runtime::{CallContext, KMap, KNumber, KValue},
};
use slab::Slab;

#[repr(C)]
pub struct KotoInterface {
    // Module Builder
    create_str: unsafe extern "C" fn(*mut c_void, *const c_char) -> ValueId,
    create_number: unsafe extern "C" fn(*mut c_void, f64) -> ValueId,
    create_bool: unsafe extern "C" fn(*mut c_void, c_char) -> ValueId,
    create_map: unsafe extern "C" fn(*mut c_void) -> ValueId,
    map_insert: unsafe extern "C" fn(*mut c_void, ValueId, *const c_char, ValueId) -> Bool,

    // Call Context
    // arg_count: unsafe extern "C" fn(*mut c_void) -> u32,
    // arg_type: unsafe extern "C" fn(*mut c_void, u32) -> u32,
    // arg_string: unsafe extern "C" fn(*mut c_void, u32) -> *const c_char,
    // arg_number: unsafe extern "C" fn(*mut c_void, u32) -> f64,
    // return_string: unsafe extern "C" fn(*mut c_void, *const c_char),
    // return_number: unsafe extern "C" fn(*mut c_void, f64),
}

impl KotoInterface {
    pub fn new() -> Self {
        Self {
            create_str: koto_create_str,
            create_number: koto_create_number,
            create_bool: koto_create_bool,
            create_map: koto_create_map,
            map_insert: koto_map_insert,

        }
    }
}

impl Default for KotoInterface {
    fn default() -> Self {
        Self::new()
    }
}

pub type LoadFunc = unsafe extern "C" fn(*const KotoInterface, module: *mut ModuleBuilder) -> ValueId;
pub type NativeFunction = extern "C" fn(*const KotoInterface, ctx: *mut CallContext) -> Bool;
pub type ValueId = c_int;
pub type Bool = c_int;

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn koto_create_str(module: *mut c_void, value: *const c_char) -> ValueId {
    if module.is_null() || value.is_null() {
        return 0;
    }

    let module = unsafe { &mut *(module as *mut ModuleBuilder) };
    let c_str = unsafe { CStr::from_ptr(value) };
    match c_str.to_str() {
        Ok(str_val) => module.create_str(str_val),
        Err(_) => 0,
    }
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn koto_create_bool(module: *mut c_void, value: c_char) -> ValueId {
    if module.is_null() {
        return 0;
    }

    let module = unsafe { &mut *(module as *mut ModuleBuilder) };
    let bool_value = value != 0;
    module.values.insert(KValue::Bool(bool_value)) as ValueId
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn koto_create_number(module: *mut c_void, value: f64) -> ValueId {
    if module.is_null() {
        return 0;
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
        return 0;
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
) -> Bool {
    if module.is_null() || key.is_null() {
        return 0;
    }

    let module = unsafe { &mut *(module as *mut ModuleBuilder) };
    let c_str = unsafe { CStr::from_ptr(key) };

    let Ok(key_str) = c_str.to_str() else {
        return 0;
    };

    if module.map_insert(map, key_str, value).is_err() {
        return 0;
    }

    1
}



/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_ctx_arg_count(ctx: *mut c_void) -> u32 {
    if ctx.is_null() {
        return 0;
    }

    let ctx = unsafe { &mut *(ctx as *mut CallContext) };
    ctx.args().len() as u32
}

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
