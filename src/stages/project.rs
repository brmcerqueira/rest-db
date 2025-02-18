use v8::{Array, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

use crate::utils::{array_update, bind, get_function};

pub fn project(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let function = bind(scope, args.get(0).try_into().unwrap(), array.into()).unwrap();

    let result: Local<Array> = get_function(scope, array.into(), "map").unwrap()
        .call(scope, array.into(), &[function.into()])
        .unwrap().try_into().unwrap();

    array_update(scope, array, result);
}
