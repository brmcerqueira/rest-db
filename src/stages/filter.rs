use v8::{Array, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

use crate::utils::{array_update, get_function, out_array};

pub fn filter(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array = out_array(scope, &args).unwrap();

    let result: Local<Array> = get_function(scope, array.into(), "filter")
        .unwrap()
        .call(scope, array.into(), &[args.get(0)])
        .unwrap()
        .try_into()
        .unwrap();

    array_update(scope, array, result);
}
