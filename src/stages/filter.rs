use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

use crate::utils::{array_update, get_function, out_array, throw_error};

pub fn filter(root_scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    throw_error(root_scope, |scope| {
        let array = out_array(&args).unwrap();

        let result = get_function(scope, array.into(), "filter")?
            .call(scope, array.into(), &[args.get(0)])
            .ok_or("can't filter")?
            .try_into();

        if result.is_ok() {
            array_update(scope, array, result.unwrap());
        }

        Ok(())
    });
}
