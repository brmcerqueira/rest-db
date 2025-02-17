use v8::{json, Array, Function, HandleScope, Integer, Local, Object};
use crate::repository::REPOSITORY;

pub fn get_function<'s, 'a>(scope: &mut HandleScope<'s>, object: Local<'a, Object>, name: &str) -> Local<'a, Function> where 's : 'a {
    let function_name = v8::String::new(scope, name)
    .expect("failed to convert Rust string to javascript string");

    let function = object.get(scope, function_name.into())
    .expect(&*format!("could not find function {name}"));

    function.try_into().unwrap()
}

pub fn array_update(scope: &mut HandleScope, array: Local<Array>, new_data: Local<Array>) {
    let length = v8::String::new(scope, "length").unwrap();

    let clear = Integer::new(scope, 0);

    array.set(scope, length.into(), clear.into());

    let push = get_function(scope, array.into(), "push");

    let length = new_data.length();

    for i in 0..length {
        let item = new_data.get_index(scope, i).unwrap();
        push.call(scope, array.into(), &[item]);
    }
}

pub fn collection_load(scope: &mut HandleScope, collection: String, array: Local<Array>) {
    REPOSITORY.get_all(collection, |item| {
        let value = v8::String::new(scope, &item).unwrap().into();
        let value = json::parse(scope, value).unwrap().into();
        array.set_index(scope, array.length(), value);
    });
}