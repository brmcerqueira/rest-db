use v8::{undefined, Array, Function, FunctionCallbackArguments, HandleScope, Integer, Local, Number, ReturnValue};

use crate::utils::{bind, get_function};

pub fn sum(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let initial_value = Integer::new(scope, 0);

    let wrapper_function = Function::new(scope, wrapper).unwrap();

    let wrapper_function = bind(scope, wrapper_function, args.get(0));

    return_value.set(
        get_function(scope, array.into(), "reduce")
            .call(
                scope,
                array.into(),
                &[wrapper_function.into(), initial_value.into()],
            )
            .unwrap()
            .try_into()
            .unwrap(),
    );
}

fn wrapper(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    let callback: Local<Function> = args.this().try_into().unwrap();
    let recv = undefined(scope);
    let accumulator = args
        .get(0)
        .to_number(scope)
        .unwrap()
        .number_value(scope)
        .unwrap();
    let current_value = callback
        .call(scope, recv.into(), &[args.get(1)])
        .unwrap()
        .to_number(scope)
        .unwrap()
        .number_value(scope)
        .unwrap();
    return_value.set(Number::new(scope, accumulator + current_value).into());
}
