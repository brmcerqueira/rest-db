use v8::{undefined, Array, Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

use crate::utils::get_function;

pub fn assign(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let function: Local<Function> = args.get(0).try_into().unwrap();

    let object_name = v8::String::new(scope, "Object").unwrap();

    let object = scope.get_current_context().global(scope).get(scope, object_name.into()).unwrap().to_object(scope).unwrap();

    let assign_function = get_function(scope, object, "assign");

    let length = array.length();

    let recv = undefined(scope);

    for i in 0..length {
        let item = array.get_index(scope, i).unwrap();

        let result = function.call(scope, recv.into(), &[item]).unwrap();

        assign_function.call(scope, object.into(), &[item, result]);
    }
}
