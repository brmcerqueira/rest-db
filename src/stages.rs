use v8::{FunctionCallbackArguments, FunctionTemplate, HandleScope, Local, ObjectTemplate, ReturnValue};

macro_rules! set_global_function {
    ($name:expr, $cb:expr) => {
        pub fn global_functions<'s>(scope: &mut HandleScope<'s, ()>, global: Local<'s, ObjectTemplate>) {
            global.set(
                v8::String::new(scope, $name).unwrap().into(),
                FunctionTemplate::new(scope, $cb).into(),
            );
        }
    };
}

set_global_function!("filter", filter);

fn filter(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
    let message = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    println!("Logged: {}", message);
}