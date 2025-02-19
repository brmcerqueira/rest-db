use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

use crate::utils::{collection_load, out_array};

pub fn collection(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array = out_array(&args).unwrap();

    let collection = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    collection_load(scope, collection, array);
}
