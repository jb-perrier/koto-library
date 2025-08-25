use std::ffi::{c_int, c_uint, c_void};

use koto::runtime::{CallContext, KNumber, KValue};

use crate::{Number, ValueType, kvalue_to_uint};

#[repr(C)]
pub struct CallContextInterface {
    arg_count: unsafe extern "C" fn(*mut c_void) -> c_uint,
    arg_type: unsafe extern "C" fn(*mut c_void, c_uint) -> ValueType,
    // arg_string: unsafe extern "C" fn(*mut c_void, c_uint) -> *const c_char,
    arg_number: unsafe extern "C" fn(*mut c_void, c_uint) -> f64,
    // return_string: unsafe extern "C" fn(*mut c_void, *const c_char),
    return_number: unsafe extern "C" fn(*mut c_void, Number),
}

impl CallContextInterface {
    pub fn new() -> Self {
        Self {
            arg_count: call_ctx_arg_count,
            arg_type: call_ctx_arg_type,
            arg_number: call_ctx_arg_number,
            return_number: call_ctx_return_number,
        }
    }
}

impl Default for CallContextInterface {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CallContextWrapper<'a, 'b> {
    ctx: &'a mut CallContext<'b>,
    return_value: Option<KValue>,
}

impl<'a, 'b> CallContextWrapper<'a, 'b> {
    pub fn new(ctx: &'a mut CallContext<'b>) -> Self {
        Self {
            ctx,
            return_value: None,
        }
    }

    pub fn args(&self) -> &[KValue] {
        self.ctx.args()
    }

    pub fn take_return_value(&mut self) -> Option<KValue> {
        self.return_value.take()
    }

    pub fn set_return_value(&mut self, value: KValue) {
        self.return_value = Some(value);
    }
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_ctx_arg_count(ctx: *mut c_void) -> c_uint {
    if ctx.is_null() {
        return 0;
    }
    let ctx = unsafe { &mut *(ctx as *mut CallContextWrapper) };
    let args = ctx.args();
    args.len() as c_uint
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_ctx_arg_type(ctx: *mut c_void, index: c_uint) -> ValueType {
    if ctx.is_null() {
        return 0;
    }

    let ctx = unsafe { &mut *(ctx as *mut CallContextWrapper) };
    let Some(arg) = ctx.args().get(index as usize) else {
        return -1;
    };
    kvalue_to_uint(arg)
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_ctx_arg_number(ctx: *mut c_void, index: c_uint) -> Number {
    if ctx.is_null() {
        return 0.0;
    }

    let ctx = unsafe { &mut *(ctx as *mut CallContextWrapper) };
    let Some(KValue::Number(KNumber::F64(arg))) = ctx.args().get(index as usize) else {
        println!("arg_number: argument at index {index} is not a number");
        return -1.0;
    };
    arg.to_owned()
}

/**
 * # Safety
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_ctx_return_number(ctx: *mut c_void, value: Number) {
    if ctx.is_null() {
        return;
    }

    let ctx = unsafe { &mut *(ctx as *mut CallContextWrapper) };
    ctx.set_return_value(KValue::Number(KNumber::F64(value)));
}
