use v8::{
    json, undefined, Array, Function, FunctionCallbackArguments, HandleScope, Local, Object,
    ReturnValue, String,
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
        lookup_array.set_index(scope, lookup_array.length(), value);
    });

    let recv = undefined(scope);

    let length = array.length();

    for i in 0..length {
        let item = array.get_index(scope, i).unwrap();

        println!("Item: {}", json::stringify(scope, item).unwrap().to_rust_string_lossy(scope));

        let this = Object::new(scope);

        let key = v8::String::new(scope, "item").unwrap();
        this.set(scope, key.into(), item);

        let key = v8::String::new(scope, "callback").unwrap();
        this.set(scope, key.into(), args.get(2));

        let wrapper_function = Function::new(scope, wrapper).unwrap();

        let bind = get_function(scope, wrapper_function.into(), "bind");

        let wrapper_function = bind
            .call(scope, wrapper_function.into(), &[this.into()])
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

fn wrapper(scope: &mut HandleScope, args: FunctionCallbackArguments, mut retval: ReturnValue) {
    let key = v8::String::new(scope, "item").unwrap();
    let item = args.this().get(scope, key.into()).unwrap();
    let key = v8::String::new(scope, "callback").unwrap();
    let callback: Local<Function> = args.this().get(scope, key.into()).unwrap().try_into().unwrap();
    let recv = undefined(scope);
    retval.set(callback.call(scope, recv.into(), &[item, args.get(0)]).unwrap());
}