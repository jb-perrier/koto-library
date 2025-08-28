use koto::runtime::{CallContext, KValue, KotoFunction};

use crate::{CallContextWrapper, ForeignNativeFunction, KotoInterface, Values, FAILURE};

/// # Safety
/// The caller must ensure that `func` points to a valid `ForeignNativeFunction`
pub unsafe fn to_koto_function(func: ForeignNativeFunction) -> impl KotoFunction {
    move |ctx: &mut CallContext| {
        let mut ctx_wrapper = CallContextWrapper::new(ctx);
        let ctx_ptr = &mut ctx_wrapper as *mut CallContextWrapper;
        let koto = KotoInterface::default();
        let koto_ptr = &koto as *const KotoInterface;
        let mut values = Values::new();
        let values_ptr = &mut values as *mut Values;
        let result = unsafe { func(koto_ptr, ctx_ptr, values_ptr) };
        if result.code == FAILURE {
            return Err("Foreign function call failed".into());
        }

        if result.value == -1 {
            return Ok(KValue::Null);
        }

        match values.take_value(result.value) {
            Some(value) => Ok(value),
            None => Err("Foreign function did not set a return value".into()),
        }
    }
}
