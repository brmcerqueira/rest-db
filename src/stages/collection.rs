use v8::{Array, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

use crate::repository::REPOSITORY;

pub fn collection(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
    let collection = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    let array: Local<Array> = args.this().try_into().unwrap();

    for item in REPOSITORY.get_all(collection) {
        let value = v8::String::new(scope, &item).unwrap().into();
        let value = v8::json::parse(scope, value).unwrap().into();
        array.set_index(scope, array.length(), value);
    }
}