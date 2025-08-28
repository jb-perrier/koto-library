#![allow(clippy::result_unit_err)]
use koto::runtime::KValue;
use std::ffi::{c_char, c_int};

mod call_context;
mod foreign_function;
mod values;

pub use call_context::*;
pub use foreign_function::*;
pub use values::*;

pub type LoadFunc = unsafe extern "C" fn(*const KotoInterface, *mut Values) -> CallResult;
pub type ForeignNativeFunction =
    unsafe extern "C" fn(*const KotoInterface, *mut CallContextWrapper, *mut Values) -> CallResult;
pub type ValueId = c_int;
pub type Bool = c_char;
pub type ResultCode = c_int;
pub const SUCCESS: ResultCode = 1;
pub const FAILURE: ResultCode = 0;

#[repr(C)]
pub struct CallResult {
    pub code: ResultCode,
    pub value: ValueId,
}

pub type Number = f64;
pub type ValueType = c_int;

pub fn kvalue_to_uint(value: &KValue) -> ValueType {
    match value {
        KValue::Null => 0,
        KValue::Bool(_) => 1,
        KValue::Number(_) => 2,
        KValue::Str(_) => 7,
        _ => 0,
    }
}

#[repr(C)]
pub struct KotoInterface {
    values: ValuesInterface,
    call: CallContextInterface,
}

impl KotoInterface {
    pub fn new() -> Self {
        Self {
            values: ValuesInterface::new(),
            call: CallContextInterface::new(),
        }
    }
}

impl Default for KotoInterface {
    fn default() -> Self {
        Self::new()
    }
}
