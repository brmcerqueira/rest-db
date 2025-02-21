use crate::local_array_extension::LocalArrayExtension;
use crate::try_catch_verify::TryCatchVerify;
use crate::utils::{out_array, try_or_throw};
use std::collections::HashMap;
use v8::{
    undefined, Array, DataError, Function, FunctionCallbackArguments, HandleScope, Local,
    ReturnValue, TryCatch, Value,
};

pub fn group(root_scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    try_or_throw(root_scope, |scope| {
        let try_catch = &mut TryCatch::new(scope);

        let out = out_array(&args)?;

        let (function, key_function): (Local<Function>, Option<Local<Function>>) =
            if args.length() == 2 {
                (
                    args.get(1)
                        .try_into()
                        .map_err(|x: DataError| x.to_string())?,
                    Some(
                        args.get(0)
                            .try_into()
                            .map_err(|x: DataError| x.to_string())?,
                    ),
                )
            } else {
                (
                    args.get(0)
                        .try_into()
                        .map_err(|x: DataError| x.to_string())?,
                    None,
                )
            };

        let mut map: HashMap<Local<Value>, Local<Array>> = HashMap::new();

        if key_function.is_some() {
            let key_function = key_function.unwrap();

            for index in 0..out.length() {
                let item = out
                    .get_index(try_catch, index)
                    .ok_or("can't get item in group")?;

                let key_call = key_function.call(try_catch, out.into(), &[item.into()]);

                try_catch.verify()?;

                let key = key_call.ok_or("can't get key in group")?;

                if map.contains_key(&key) {
                    let array = map.get(&key).ok_or("can't get group")?;
                    array.set_index(try_catch, array.length(), item);
                } else {
                    map.insert(key, Array::new_with_elements(try_catch, &[item]));
                }
            }
        } else {
            map.insert(undefined(try_catch).into(), out);
        }

        out.clear(try_catch);

        for item in map {
            let call = function.call(try_catch, item.1.into(), &[item.0.into()]);

            try_catch.verify()?;

            let result = call.ok_or("can't get transform in group")?;

            out.set_index(try_catch, out.length(), result);
        }

        Ok(())
    });
}
