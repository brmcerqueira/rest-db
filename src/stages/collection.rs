use crate::local_array_extension::LocalArrayExtension;
use crate::utils::{out_array, try_or_throw};
use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn collection(root_scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    try_or_throw(root_scope, |scope| {
        let out = out_array(&args)?;

        let collection = args
            .get(0)
            .to_string(scope)
            .ok_or("can't create collection name")?
            .to_rust_string_lossy(scope);

        out.collection_load(scope, collection);

        Ok(())
    });
}
