use std::ffi::{c_uint, c_void};

use koto::runtime::{CallContext, KNumber, KValue};

use crate::{Number, ValueType, kvalue_to_uint};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CallContextInterface<'a> {
    this: *mut CallContext<'a>,
    arg_count: unsafe extern "C" fn(*mut CallContext) -> c_uint,
    arg_type: unsafe extern "C" fn(*mut CallContext, c_uint) -> ValueType,
    // arg_string: unsafe extern "C" fn(*mut c_void, c_uint) -> *const c_char,
    arg_number: unsafe extern "C" fn(*mut CallContext, c_uint) -> f64,
}

impl<'a> CallContextInterface<'a> {
    pub fn new(this: &mut CallContext<'a>) -> Self {
        Self {
            this,
            arg_count: call_ctx_arg_count,
            arg_type: call_ctx_arg_type,
            arg_number: call_ctx_arg_number,
        }
    }
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_ctx_arg_count(ctx: *mut CallContext) -> c_uint {
    if ctx.is_null() {
        return 0;
    }
    let ctx = unsafe { &mut *(ctx as *mut CallContext) };
    let args = ctx.args();
    args.len() as c_uint
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_ctx_arg_type(ctx: *mut CallContext, index: c_uint) -> ValueType {
    if ctx.is_null() {
        return 0;
    }

    let ctx = unsafe { &mut *(ctx as *mut CallContext) };
    let Some(arg) = ctx.args().get(index as usize) else {
        return -1;
    };
    kvalue_to_uint(arg)
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_ctx_arg_number(ctx: *mut CallContext, index: c_uint) -> Number {
    if ctx.is_null() {
        return 0.0;
    }

    let ctx = unsafe { &mut *(ctx as *mut CallContext) };
    let Some(KValue::Number(KNumber::F64(arg))) = ctx.args().get(index as usize) else {
        println!("arg_number: argument at index {index} is not a number");
        return -1.0;
    };
    arg.to_owned()
}