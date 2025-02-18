use std::collections::HashMap;
use v8::{undefined, Array, Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue, Value};

use crate::utils::clear;

pub fn group(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let (function, key_function): (Local<Function>, Option<Local<Function>>) = if args.length() == 2 {
        (args.get(1).try_into().unwrap(), Some(args.get(0).try_into().unwrap()))
    } else { (args.get(0).try_into().unwrap(), None) };

    let mut map: HashMap<Local<Value>, Local<Array>> = HashMap::new();

    let undefined = undefined(scope);

    if key_function.is_some() {
        let key_function = key_function.unwrap();
        for index in 0..array.length() {
            let item = array.get_index(scope, index).unwrap();
            let key = key_function.call(scope, undefined.into(), &[item.into()]).unwrap();
            if map.contains_key(&key) {
                let group_array = map.get(&key).unwrap();
                group_array.set_index(scope, group_array.length(), item);
            }
            else {
                map.insert(key, Array::new_with_elements(scope, &[item]));
            }
        }
    }
    else {
        map.insert(undefined.into(), array);
    }

    clear(scope, array);

    for item in map {
        let result = function.call(scope, item.1.into(), &[item.0.into()]).unwrap();
        array.set_index(scope, array.length(), result);
    }
}
