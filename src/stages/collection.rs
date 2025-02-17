use v8::{Array, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

use crate::utils::collection_load;

pub fn collection(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let collection = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    collection_load(scope, collection, array);
}