use crate::try_catch_verify::TryCatchVerify;
use crate::utils::{get_function, out_array, try_or_throw};
use v8::{
    DataError, Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue, TryCatch,
};

pub fn assign(root_scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    try_or_throw(root_scope, |scope| {
        let try_catch = &mut TryCatch::new(scope);

        let out = out_array(&args)?;

        let function: Local<Function> = args
            .get(0)
            .try_into()
            .map_err(|x: DataError| x.to_string())?;

        let object_name =
            v8::String::new(try_catch, "Object").ok_or("can't create string Object")?;

        let object = try_catch
            .get_current_context()
            .global(try_catch)
            .get(try_catch, object_name.into())
            .ok_or("can't get Object")?
            .to_object(try_catch)
            .ok_or("can't convert to Object")?;

        let assign_function = get_function(try_catch, object, "assign")?;

        let length = out.length();

        for i in 0..length {
            let item = out
                .get_index(try_catch, i)
                .ok_or("can't get item in assign")?;

            let call = function.call(try_catch, out.into(), &[item]);

            try_catch.verify()?;

            assign_function.call(
                try_catch,
                object.into(),
                &[item, call.ok_or("can't assign new fields in item")?],
            );
        }

        Ok(())
    });
}
