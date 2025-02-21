use crate::utils::{out_array, try_or_throw};
use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn last(
    root_scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    try_or_throw(root_scope, |scope| {
        let out = out_array(&args)?;
        let item = out
            .get_index(scope, out.length() - 1)
            .ok_or("can't get last")?;
        return_value.set(item);
        Ok(())
    });
}
