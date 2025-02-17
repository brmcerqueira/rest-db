use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn result(_: &mut HandleScope, args: FunctionCallbackArguments, mut return_value: ReturnValue) {
    return_value.set(args.this().into());
}