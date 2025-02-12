use v8::{json, Array, FunctionCallbackArguments, HandleScope, Local, ReturnValue, String};

use crate::repository::REPOSITORY;

pub fn collection(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let collection = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    REPOSITORY.get_all(collection, |item| {
        let value = String::new(scope, &item).unwrap().into();
        let value = json::parse(scope, value).unwrap().into();
        array.set_index(scope, array.length(), value);
    });
}