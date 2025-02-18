use crate::utils::out_array;
use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn result(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    return_value.set(out_array(scope, &args).unwrap().into());
}
