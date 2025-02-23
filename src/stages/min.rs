use crate::utils::out_calculate_operator;
use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn min(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    return_value.set_double(out_calculate_operator(
        scope,
        &args,
        |result: &mut f64, value: f64| {
            if value < *result {
                *result = value;
            }
        },
    ));
}
