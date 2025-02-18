use std::collections::HashMap;
use v8::{undefined, Array, Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue, Value};

use crate::utils::clear;

pub fn group(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let (function, key_function): (Local<Function>, Option<Local<Function>>) = if args.length() == 2 {
        (args.get(1).try_into().unwrap(), Some(args.get(0).try_into().unwrap()))
    } else { (args.get(0).try_into().unwrap(), None) };

    let mut map: HashMap<Local<Value>, Local<Array>> = HashMap::new();

    let default = undefined(scope);

    if key_function.is_some() {
        for index in 0..array.length() {

        }
    }
    else {
        map.insert(default.into(), array);
    }

    clear(scope, array);

    for item in map {
        let result = function.call(scope, item.1.into(), &[item.0.into()]).unwrap();
        array.set_index(scope, array.length(), result);
    }
}
