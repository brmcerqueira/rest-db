use v8::{Array, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

use crate::utils;

pub fn filter(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let function = utils::get_function(scope, array.into(), "filter");

    let result = function.call(scope, array.into(), &[args.get(0)]).unwrap();

    let length = v8::String::new(scope, "length").unwrap();

    let clear = v8::Integer::new(scope, 0);

    array.set(scope, length.into(), clear.into());

    let function = utils::get_function(scope, array.into(), "push");

    function.call(scope, array.into(), &[result]).unwrap();
}