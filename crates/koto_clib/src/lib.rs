#![allow(clippy::result_unit_err)]
use koto::runtime::KValue;
use std::{ffi::{c_char, c_int}, marker::PhantomData};

mod call_context;
mod foreign_function;
mod values;

pub use call_context::*;
pub use foreign_function::*;
pub use values::*;

pub type LoadFunc = unsafe extern "C" fn(*mut ValuesInterface) -> CallResult;
pub type ForeignNativeFunction =
    unsafe extern "C" fn(*mut CallContextInterface, *mut ValuesInterface) -> CallResult;
pub type ValueId = c_int;
pub struct Value(pub ValueId);

pub struct NumberRef {
    id: ValueId,
}


pub trait ValueRef {
    fn id(&self) -> ValueId;
}

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
