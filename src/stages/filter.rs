use v8::{Array, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

use crate::utils::{array_update, get_function};

pub fn filter(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let result: Local<Array> = get_function(scope, array.into(), "filter").unwrap()
        .call(scope, array.into(), &[args.get(0)])
        .unwrap()
        .try_into()
        .unwrap();

    array_update(scope, array, result);
}
