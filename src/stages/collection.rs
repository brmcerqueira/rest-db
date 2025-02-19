use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};
use crate::local_array_extension::LocalArrayExtension;
use crate::utils::out_array;

pub fn collection(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array = out_array(&args).unwrap();

    let collection = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    array.collection_load(scope, collection);
}
