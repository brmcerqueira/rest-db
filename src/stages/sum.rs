use crate::try_catch_verify::TryCatchVerify;
use crate::utils::{out_array, try_or_throw};
use v8::{undefined, DataError, Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue, TryCatch};

pub fn sum(
    root_scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut return_value: ReturnValue,
) {
    try_or_throw(root_scope, |scope| {
        let try_catch = &mut TryCatch::new(scope);

        let out = out_array(&args)?;

        let mut accumulator = 0f64;

        let function: Local<Function> = args
            .get(0)
            .try_into()
            .map_err(|x: DataError| x.to_string())?;

        let recv = undefined(try_catch);

        for index in 0..out.length() {
            let item = out
                .get_index(try_catch, index)
                .ok_or("can't get item in sum")?;

            let call = function.call(try_catch, recv.into(), &[item]);

            try_catch.verify()?;

            accumulator += call.ok_or("can't get value from item in sum")?
                .to_number(try_catch)
                .ok_or("can't convert local number from value in sum")?
                .number_value(try_catch)
                .ok_or("can't convert number from value in sum")?
        }

        return_value.set_double(accumulator);

        Ok(())
    });
}