use v8::{
    json, undefined, Array, Function, FunctionCallbackArguments, HandleScope,
    Local, ReturnValue, String,
};

use crate::{repository::REPOSITORY, utils::get_function};

pub fn lookup(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
    let array: Local<Array> = args.this().try_into().unwrap();

    let collection = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    let lookup_array = Array::new(scope, 0);

    REPOSITORY.get_all(collection, |item| {
        let value = String::new(scope, &item).unwrap().into();
        let value = json::parse(scope, value).unwrap().into();
        lookup_array.set_index(scope, array.length(), value);
    });

    let recv = undefined(scope);

    let length = array.length();

    for i in 0..length {
        let item = array.get_index(scope, i).unwrap();

        let function: Local<Function> = args.get(2).try_into().unwrap();

        let wrapper_function = Function::new(
            scope,
            |scope1: &mut HandleScope,
             args1: FunctionCallbackArguments,
             mut _retval1: ReturnValue| {
                let recv = undefined(scope1);
                _retval1.set(item);
                //function.call(scope1, recv.into(), &[args1.get(0)]);
            },
        )
        .unwrap();

        let result = get_function(scope, lookup_array.into(), "filter")
            .call(scope, lookup_array.into(), &[wrapper_function.into()])
            .unwrap();

        let destiny = args.get(1);

        if destiny.is_string() {
            item.to_object(scope).unwrap().set(scope, destiny, result);
        } else {
            let function: Local<Function> = destiny.try_into().unwrap();
            function.call(scope, recv.into(), &[item, result]).unwrap();
        }
    }
}
