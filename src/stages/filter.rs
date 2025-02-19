use crate::try_catch_verify::TryCatchVerify;
use crate::utils::{out_array, try_or_throw};
use std::collections::VecDeque;
use v8::{undefined, DataError, Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue, TryCatch};
use crate::local_array_extension::LocalArrayExtension;

pub fn filter(root_scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    try_or_throw(root_scope, |scope| {
        let try_catch = &mut TryCatch::new(scope);

        let array = out_array(&args)?;

        let function: Local<Function> = args.get(0).try_into().map_err(|x: DataError| x.to_string())?;

        let recv = undefined(try_catch);

        let mut queue: VecDeque<u32> = VecDeque::new();

        let mut index = 0;
        let mut removed = 0;
        while index < array.length() {
            let item = array.get_index(try_catch, index).ok_or("can't get item")?;

            println!("item: {:?}", item.to_rust_string_lossy(try_catch));

            let call = function.call(try_catch, recv.into(), &[item]);

            try_catch.verify()?;

            if call.ok_or("can't filter")?.to_boolean(try_catch).boolean_value(try_catch) {
                if let Some(new_index) = queue.pop_front() {
                    array.delete_index(try_catch, index);
                    queue.push_back(index);
                    array.set_index(try_catch, new_index, item);
                }
            }
            else {
                array.delete_index(try_catch, index);
                queue.push_back(index);
                removed += 1;
            }

            index += 1;
        }

        array.set_length(try_catch, (index - removed) as i32);

        Ok(())
    });
}
