use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn filter(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
    let message = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    println!("Logged: {} This: {}", message, args.this().to_string(scope).unwrap().to_rust_string_lossy(scope));
}