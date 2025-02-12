use v8::{Array, Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

pub fn filter(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let name = v8::String::new(scope, "filter").unwrap();

    let function = array.get(scope, name.into()).unwrap();

    let function: Local<Function> = function.try_into().unwrap();

    let result = function.call(scope, array.into(), &[args.get(0)]).unwrap();

    let length = v8::String::new(scope, "length").unwrap();

    let clear = v8::Integer::new(scope, 0);

    array.set(scope, length.into(), clear.into());

    let name = v8::String::new(scope, "push").unwrap();

    let function = array.get(scope, name.into()).unwrap();

    let function: Local<Function> = function.try_into().unwrap();

    function.call(scope, array.into(), &[result]).unwrap();
}