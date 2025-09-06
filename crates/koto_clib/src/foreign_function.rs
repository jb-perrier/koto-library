use koto::runtime::{CallContext, KValue, KotoFunction};

use crate::{ForeignNativeFunction, Values, ValuesInterface, FAILURE};

pub fn to_koto_function(func: ForeignNativeFunction) -> impl KotoFunction {
    move |ctx: &mut CallContext| {
        let mut ctx_inter = crate::CallContextInterface::new(ctx);
        let mut values = Values::new();
        let mut values_inter = ValuesInterface::new(&mut values);
        let result = unsafe { func(&mut ctx_inter, &mut values_inter) };
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

