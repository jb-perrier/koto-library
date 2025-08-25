#![allow(clippy::result_unit_err)]
use std::
    ffi::{c_char, c_int}
;
use koto::
    runtime::KValue
;

mod module_builder;
mod foreign_function;
mod call_context;

pub use call_context::*;
pub use foreign_function::*;
pub use module_builder::*;

pub type LoadFunc = unsafe extern "C" fn(*const KotoInterface, *mut ModuleBuilder) -> ValueId;
pub type ForeignNativeFunction = unsafe extern "C" fn(*const KotoInterface, *mut CallContextWrapper) -> ResultCode;
pub type ValueId = c_int;
pub type Bool = c_char;
pub type ResultCode = c_int;
const SUCCESS: ResultCode = 1;
const FAILURE: ResultCode = 0;

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
    module: ModuleBuilderInterface,
    call: CallContextInterface,
}

impl KotoInterface {
    pub fn new() -> Self {
        Self {
            module: ModuleBuilderInterface::new(),
            call: CallContextInterface::new()
        }
    }
}

impl Default for KotoInterface {
    fn default() -> Self {
        Self::new()
    }
}

