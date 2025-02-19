use crate::try_catch_verify::TryCatchVerify;
use crate::utils::{out_array, try_or_throw};
use v8::{
    DataError, Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue, TryCatch,
};

pub fn project(root_scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    try_or_throw(root_scope, |scope| {
        let try_catch = &mut TryCatch::new(scope);
        let out = out_array(&args)?;

        let function: Local<Function> = args
            .get(0)
            .try_into()
            .map_err(|x: DataError| x.to_string())?;

        for index in 0..out.length() {
            let item = out
                .get_index(try_catch, index)
                .ok_or("can't get item in project")?;

            let call = function.call(try_catch, out.into(), &[item]);

            try_catch.verify()?;

            out.set_index(try_catch, index, call.ok_or("can't project")?);
        }

        Ok(())
    });
}
