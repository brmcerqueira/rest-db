use crate::utils::{out_array, try_or_throw};
use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn count(
    root_scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    try_or_throw(root_scope, |_| {
        return_value.set_uint32(out_array(&args)?.length());
        Ok(())
    });
}
