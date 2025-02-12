use v8::{Function, HandleScope, Local, Object};

pub fn get_function<'s, 'a>(scope: &mut HandleScope<'s>, object: Local<'a, Object>, name: &str) -> Local<'a, Function> where 's : 'a {
    let function_name = v8::String::new(scope, name)
    .expect("failed to convert Rust string to javascript string");

    let function = object.get(scope, function_name.into())
    .expect(&*format!("could not find function {name}"));

    return function.try_into().unwrap();
}