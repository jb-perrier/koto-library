use std::ffi::{CStr, c_char, c_void};

use koto::{
    parser::KString,
    runtime::{KMap, KNativeFunction, KNumber, KValue},
};
use slab::Slab;

use crate::{Bool, FAILURE, ForeignNativeFunction, ResultCode, SUCCESS, ValueId, to_koto_function};

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

    /// # Safety
    /// The caller must ensure that `func` points to a valid `ForeignNativeFunction`
    pub unsafe fn create_native_function(&mut self, func: ForeignNativeFunction) -> ValueId {
        let koto_function = unsafe { to_koto_function(func) };
        self.values
            .insert(KValue::NativeFunction(KNativeFunction::new(koto_function))) as ValueId
    }

    pub fn create_map(&mut self) -> ValueId {
        self.values.insert(KValue::Map(KMap::default())) as ValueId
    }

    pub fn map_insert(&mut self, map_id: ValueId, key: &str, value_id: ValueId) -> Result<(), ()> {
        let Some(value) = self.values.try_remove(value_id as usize) else {
            return Err(());
        };

        if let Some(KValue::Map(map)) = self.values.get_mut(map_id as usize) {
            map.insert(key, value);
            Ok(())
        } else {
            self.values.insert(value);
            Err(())
        }
    }

    pub fn take_value(&mut self, id: ValueId) -> Option<KValue> {
        self.values.try_remove(id as usize)
    }
}

#[repr(C)]
pub struct ModuleBuilderInterface {
    create_str: unsafe extern "C" fn(*mut c_void, *const std::os::raw::c_char) -> ValueId,
    create_number: unsafe extern "C" fn(*mut c_void, f64) -> ValueId,
    create_bool: unsafe extern "C" fn(*mut c_void, Bool) -> ValueId,
    create_map: unsafe extern "C" fn(*mut c_void) -> ValueId,
    create_native_function: unsafe extern "C" fn(*mut c_void, *mut c_void) -> ValueId,
    map_insert: unsafe extern "C" fn(*mut c_void, ValueId, *const c_char, ValueId) -> ResultCode,
}

impl ModuleBuilderInterface {
    pub fn new() -> Self {
        Self {
            create_str,
            create_number,
            create_bool,
            create_map,
            create_native_function,
            map_insert,
        }
    }
}

impl Default for ModuleBuilderInterface {
    fn default() -> Self {
        Self::new()
    }
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
unsafe extern "C" fn create_str(module: *mut c_void, value: *const c_char) -> ValueId {
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
unsafe extern "C" fn create_bool(module: *mut c_void, value: c_char) -> ValueId {
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
unsafe extern "C" fn create_number(module: *mut c_void, value: f64) -> ValueId {
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
unsafe extern "C" fn create_native_function(module: *mut c_void, func: *mut c_void) -> ValueId {
    if module.is_null() || func.is_null() {
        return -1;
    }

    let module = unsafe { &mut *(module as *mut ModuleBuilder) };
    let func: ForeignNativeFunction = unsafe { std::mem::transmute(func) };
    unsafe { module.create_native_function(func) }
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
unsafe extern "C" fn create_map(module: *mut c_void) -> ValueId {
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
unsafe extern "C" fn map_insert(
    module: *mut c_void,
    map: ValueId,
    key: *const c_char,
    value: ValueId,
) -> ResultCode {
    if module.is_null() || key.is_null() {
        return FAILURE;
    }

    let module = unsafe { &mut *(module as *mut ModuleBuilder) };
    let c_str = unsafe { CStr::from_ptr(key) };

    let Ok(key_str) = c_str.to_str() else {
        return FAILURE;
    };

    if module.map_insert(map, key_str, value).is_err() {
        return FAILURE;
    }

    SUCCESS
}
