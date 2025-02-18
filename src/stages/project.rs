use v8::{Array, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

use crate::utils::{array_update, bind, get_function, out_array};

pub fn project(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array = out_array(scope, &args).unwrap();

    let function = bind(scope, args.get(0).try_into().unwrap(), array.into()).unwrap();

    let result: Local<Array> = get_function(scope, array.into(), "map")
        .unwrap()
        .call(scope, array.into(), &[function.into()])
        .unwrap()
        .try_into()
        .unwrap();

    array_update(scope, array, result);
}
