use v8::{json, Array, Exception, Function, FunctionCallbackArguments, HandleScope, Integer, Local, Object, Value};
use crate::repository::REPOSITORY;

pub fn get_function<'s, 'a>(scope: &mut HandleScope<'s>, object: Local<'a, Object>, name: &str) -> Result<Local<'a, Function>, String> where 's : 'a {
    let function_name = v8::String::new(scope, name).unwrap();

    let function = object.get(scope, function_name.into()).map(|value| {
        if value.is_undefined() {
            Err(throw(scope, &*format!("could not find function -> {name}")))
        }
        else {
            Ok(value)
        }
    }).unwrap();

    Ok(function?.try_into().unwrap())
}

pub fn out_array<'s, 'a>(scope: &mut HandleScope, args: &FunctionCallbackArguments<'a>) -> Result<Local<'a, Array>, String> where 's : 'a {
    if args.this().is_array() {
        Ok(args.this().try_into().unwrap())
    }
    else {
        Err(throw(scope, "failed to found a context"))
    }
}

pub fn throw(scope: &mut HandleScope, message: &str) -> String {
    let value = v8::String::new(scope, message).unwrap();
    let exception = Exception::error(scope, value);
    scope.throw_exception(exception);
    message.to_string()
}

pub fn bind<'s, 'a>(scope: &mut HandleScope<'s>, function: Local<'a, Function>, this: Local<'a, Value>) -> Result<Local<'a, Function>, String> where 's : 'a {
    let bind = get_function(scope, function.into(), "bind")?;
    Ok(bind.call(scope, function.into(), &[this]).unwrap().try_into().unwrap())
}

pub fn array_update(scope: &mut HandleScope, array: Local<Array>, new_data: Local<Array>) {
    clear(scope, array);

    for index in 0..new_data.length() {
        let item = new_data.get_index(scope, index).unwrap();
        array.set_index(scope, index, item);
    }
}

pub fn clear(scope: &mut HandleScope, array: Local<Array>) {
    let length = v8::String::new(scope, "length").unwrap();
    let value = Integer::new(scope, 0);
    array.set(scope, length.into(), value.into());
}

pub fn collection_load(scope: &mut HandleScope, collection: String, array: Local<Array>) {
    REPOSITORY.get_all(collection, |item| {
        let value = v8::String::new(scope, &item).unwrap().into();
        let value = json::parse(scope, value).unwrap().into();
        array.set_index(scope, array.length(), value);
    });
}

pub fn copy(scope: &mut HandleScope, origin_array: Local<Array>, array: Local<Array>) {
    for index in 0..origin_array.length() {
        let value = origin_array.get_index(scope, index).unwrap();
        array.set_index(scope, index, value);
    }
}