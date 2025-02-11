use v8::{FunctionCallbackArguments, FunctionTemplate, HandleScope, Local, ObjectTemplate, ReturnValue};

macro_rules! set_global_function {
    ($global:expr, $scope:expr, $name:expr, $cb:expr) => {{
        $global.set(
            v8::String::new($scope, $name).unwrap().into(),
            FunctionTemplate::new($scope, $cb).into(),
        );
    }};
}

pub fn global_functions<'s>(scope: &mut HandleScope<'s, ()>, global: Local<'s, ObjectTemplate>) {
    set_global_function!(global, scope, "filter", filter);
}

fn filter(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
    let message = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    println!("Logged: {}", message);
}