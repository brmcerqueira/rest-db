use crate::local_array_extension::LocalArrayExtension;
use crate::try_catch_verify::TryCatchVerify;
use crate::utils::{out_array, try_or_throw};
use v8::{
    Array, DataError, Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue,
    TryCatch,
};

pub fn lookup(root_scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    try_or_throw(root_scope, |scope| {
        let try_catch = &mut TryCatch::new(scope);

        let out = out_array(&args)?;

        let collection = args
            .get(0)
            .to_string(try_catch)
            .ok_or("can't create collection name")?
            .to_rust_string_lossy(try_catch);

        let lookup_array = Array::new(try_catch, 0);

        lookup_array.collection_load(try_catch, collection);

        let function: Option<Local<Function>> = if args.length() == 3 {
            Some(
                args.get(2)
                    .try_into()
                    .map_err(|x: DataError| x.to_string())?,
            )
        } else {
            None
        };

        for index in 0..out.length() {
            let array = Array::new(try_catch, 0);

            array.copy(try_catch, lookup_array);

            let item = out
                .get_index(try_catch, index)
                .ok_or("can't get item in lookup")?;

            if let Some(function) = function {
                let call = function.call(try_catch, array.into(), &[item]);

                try_catch.verify()?;

                call.ok_or("can't call scope function in lookup")?;
            }

            let destiny = args.get(1);

            if destiny.is_string() {
                item.to_object(try_catch)
                    .ok_or("can't convert item to object")?
                    .set(try_catch, destiny, array.into());
            } else {
                let function: Local<Function> =
                    destiny.try_into().map_err(|x: DataError| x.to_string())?;

                let call = function.call(try_catch, array.into(), &[item]);

                try_catch.verify()?;

                call.ok_or("can't call destiny function in lookup")?;
            }
        }

        Ok(())
    });
}
