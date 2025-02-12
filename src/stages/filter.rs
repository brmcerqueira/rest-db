use v8::{Array, FunctionCallbackArguments, HandleScope, Integer, Local, ReturnValue, String};

use crate::utils::get_function;

pub fn filter(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let result: Local<Array> = get_function(scope, array.into(), "filter")
        .call(scope, array.into(), &[args.get(0)])
        .unwrap().try_into().unwrap();

    let length = String::new(scope, "length").unwrap();

    let clear = Integer::new(scope, 0);

    array.set(scope, length.into(), clear.into());

    let push = get_function(scope, array.into(), "push");

    let length = result.length();

    for i in 0..length {
        let item = result.get_index(scope, i).unwrap();
        push.call(scope, array.into(), &[item]);
    }
}
