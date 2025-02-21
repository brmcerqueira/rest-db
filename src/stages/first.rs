use crate::utils::{out_array, try_or_throw};
use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn first(
    root_scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    try_or_throw(root_scope, |scope| {
        let item = out_array(&args)?
            .get_index(scope, 0)
            .ok_or("can't get first")?;
        return_value.set(item);
        Ok(())
    });
}
