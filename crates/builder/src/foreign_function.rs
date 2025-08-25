use koto::runtime::{CallContext, KotoFunction};

use crate::{CallContextWrapper, ForeignNativeFunction, KotoInterface};

/// # Safety
/// The caller must ensure that `func` points to a valid `ForeignNativeFunction`
pub unsafe fn to_koto_function(func: ForeignNativeFunction) -> impl KotoFunction {
    move |ctx: &mut CallContext| {
        let mut ctx_wrapper = CallContextWrapper::new(ctx);
        let ctx_ptr = &mut ctx_wrapper as *mut CallContextWrapper;
        let koto = KotoInterface::default();
        let koto_ptr = &koto as *const KotoInterface;
        let is_success = unsafe { func(koto_ptr, ctx_ptr) };
        if is_success == 0 {
            return Err("Foreign function call failed".into());
        }

        match ctx_wrapper.take_return_value() {
            Some(value) => Ok(value),
            None => Err("Foreign function did not set a return value".into()),
        }
    }
}
