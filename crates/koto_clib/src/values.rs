use std::ffi::{CStr, c_char, c_void};

use koto::{
    parser::KString,
    runtime::{KMap, KNativeFunction, KNumber, KValue},
};
use slab::Slab;

use crate::{to_koto_function, ForeignNativeFunction, ResultCode, Value, ValueId, FAILURE, SUCCESS};

#[derive(Default)]
pub struct Values {
    pub values: Slab<KValue>,
}

impl Values {
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
#[derive(Debug, Clone, Copy)]
pub struct ValuesInterface {
    this: *mut Values,
    create_null: unsafe extern "C" fn(*mut Values) -> ValueId,
    create_str: unsafe extern "C" fn(*mut Values, *const std::os::raw::c_char) -> ValueId,
    create_number: unsafe extern "C" fn(*mut Values, f64) -> ValueId,
    create_bool: unsafe extern "C" fn(*mut Values, c_char) -> ValueId,
    create_map: unsafe extern "C" fn(*mut Values) -> ValueId,
    create_native_function: unsafe extern "C" fn(*mut Values, ForeignNativeFunction) -> ValueId,
    map_insert: unsafe extern "C" fn(*mut Values, ValueId, *const c_char, ValueId) -> ResultCode,
}

impl ValuesInterface {
    pub fn new(this: &mut Values) -> Self {
        Self {
            this,
            create_null,
            create_str,
            create_number,
            create_bool,
            create_map,
            create_native_function,
            map_insert,
        }
    }

    pub fn number(&mut self, num: f64) -> Value {
        Value(unsafe { (self.create_number)(self.this, num) })
    }

    pub fn null(&mut self) -> Value {
        Value(unsafe { (self.create_null)(self.this) })
    }

    pub fn bool(&mut self, b: bool) -> Value {
        let c_bool: c_char = if b { 1 } else { 0 };
        Value(unsafe { (self.create_bool)(self.this, c_bool) })
    }

    /// Prefer 'string_from_cstr' when possible
    pub fn string(&mut self, s: &str) -> Value {
        let c_string = std::ffi::CString::new(s).unwrap_or_default();
        Value(unsafe { (self.create_str)(self.this, c_string.as_ptr()) })
    }

    /// Fastest way to create a string
    pub fn string_from_cstr(&mut self, c_str: &CStr) -> Value {
        Value(unsafe { (self.create_str)(self.this, c_str.as_ptr()) })
    }

    pub fn native_function(&mut self, func: ForeignNativeFunction) -> Value {
        Value(unsafe { (self.create_native_function)(self.this, func) })
    }

    pub fn map(&mut self) -> anyhow::Result<Value> {
        let map_id = unsafe { (self.create_map)(self.this) };
        if map_id == -1 {
            Err(anyhow::anyhow!("Failed to create map"))
        } else {
            Ok(Value(map_id))
        }
    }

    pub fn map_insert(&mut self, map: &Value, key: &str, value: Value) -> anyhow::Result<()> {
        let c_string = std::ffi::CString::new(key).map_err(|_| anyhow::anyhow!("Invalid key"))?;
        let result = unsafe { (self.map_insert)(self.this, map.0, c_string.as_ptr(), value.0) };
        if result == SUCCESS {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to insert into map"))
        }
    }
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
unsafe extern "C" fn create_null(values: *mut Values) -> ValueId {
    if values.is_null() {
        return 0;
    }

    let values = unsafe { &mut *values };
    values.values.insert(KValue::Null) as ValueId
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
unsafe extern "C" fn create_str(values: *mut Values, value: *const c_char) -> ValueId {
    if values.is_null() || value.is_null() {
        return 0;
    }

    let values = unsafe { &mut *values };
    let c_str = unsafe { CStr::from_ptr(value) };
    match c_str.to_str() {
        Ok(str_val) => values.create_str(str_val),
        Err(_) => 0,
    }
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
unsafe extern "C" fn create_bool(values: *mut Values, value: c_char) -> ValueId {
    if values.is_null() {
        return 0;
    }

    let values = unsafe { &mut *values };
    let bool_value = value != 0;
    values.values.insert(KValue::Bool(bool_value)) as ValueId
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
unsafe extern "C" fn create_number(values: *mut Values, value: f64) -> ValueId {
    if values.is_null() {
        return 0;
    }

    let values = unsafe { &mut *values };
    values.create_number(value)
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
unsafe extern "C" fn create_native_function(values: *mut Values, func: ForeignNativeFunction) -> ValueId {
    if values.is_null() {
        return -1;
    }

    let values = unsafe { &mut *values };
    unsafe { values.create_native_function(func) }
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
unsafe extern "C" fn create_map(values: *mut Values) -> ValueId {
    if values.is_null() {
        return -1;
    }

    let values = unsafe { &mut *values };
    values.create_map()
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
unsafe extern "C" fn map_insert(
    values: *mut Values,
    map: ValueId,
    key: *const c_char,
    value: ValueId,
) -> ResultCode {
    if values.is_null() || key.is_null() {
        return FAILURE;
    }

    let values = unsafe { &mut *(values as *mut Values) };
    let c_str = unsafe { CStr::from_ptr(key) };

    let Ok(key_str) = c_str.to_str() else {
        return FAILURE;
    };

    if values.map_insert(map, key_str, value).is_err() {
        return FAILURE;
    }

    SUCCESS
}
