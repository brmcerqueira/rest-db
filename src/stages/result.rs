use crate::utils::out_array;
use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn result(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    if let Ok(array) = out_array(scope, &args) {
        return_value.set(array.into());
    }
}
