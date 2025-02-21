use crate::utils::{out_array, try_or_throw};
use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn all(
    root_scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    try_or_throw(root_scope, |_| {
        return_value.set(out_array(&args)?.into());
        Ok(())
    });
}
