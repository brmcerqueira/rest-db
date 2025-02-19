use crate::utils::{out_array, throw_error};
use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn result(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    throw_error(scope, |_| {
        return_value.set(out_array(&args)?.into());
        Ok(())
    });
}
