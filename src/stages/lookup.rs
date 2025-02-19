use crate::local_array_extension::LocalArrayExtension;
use crate::utils::out_array;
use v8::{undefined, Array, Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

pub fn lookup(scope: &mut HandleScope, args: FunctionCallbackArguments, _: ReturnValue) {
    let array = out_array(&args).unwrap();

    let collection = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    let origin_array = Array::new(scope, 0);

    origin_array.collection_load(scope, collection);

    let function: Option<Local<Function>> = if args.length() == 3 {
        Some(args.get(2).try_into().unwrap())
    } else {
        None
    };

    let recv = undefined(scope);

    for index in 0..array.length() {
        let lookup_array = Array::new(scope, 0);

        lookup_array.copy(scope, origin_array);

        let item = array.get_index(scope, index).unwrap();

        if let Some(function) = function {
            function.call(scope, lookup_array.into(), &[item]).unwrap();
        }

        let destiny = args.get(1);

        if destiny.is_string() {
            item.to_object(scope)
                .unwrap()
                .set(scope, destiny, lookup_array.into());
        } else {
            let function: Local<Function> = destiny.try_into().unwrap();
            function
                .call(scope, recv.into(), &[item, lookup_array.into()])
                .unwrap();
        }
    }
}
