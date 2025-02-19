use crate::repository::REPOSITORY;
use v8::{
    json, Array, Exception, Function, FunctionCallbackArguments, HandleScope, Integer, Local,
    Object, Value,
};

pub fn get_function<'s, 'a>(
    scope: &mut HandleScope<'s>,
    object: Local<'a, Object>,
    name: &str,
) -> Result<Local<'a, Function>, String>
where
    's: 'a,
{
    let function_name = v8::String::new(scope, name).unwrap();

    let function = object
        .get(scope, function_name.into())
        .map(|value| {
            if value.is_undefined() {
                Err(format!("could not find function -> {name}"))
            } else {
                Ok(value)
            }
        })
        .unwrap();

    Ok(function?.try_into().unwrap())
}

pub fn out_array<'a>(args: &FunctionCallbackArguments<'a>) -> Result<Local<'a, Array>, String> {
    if args.this().is_array() {
        Ok(args.this().try_into().unwrap())
    } else {
        Err("failed to found a context".to_string())
    }
}

pub fn throw_error(
    scope: &mut HandleScope,
    mut block: impl FnMut(&mut HandleScope) -> Result<(), String>,
) {
    if let Err(e) = block(scope) {
        let value = v8::String::new(scope, &*e).unwrap();
        let exception = Exception::error(scope, value);
        scope.throw_exception(exception);
    }
}

pub fn bind<'s, 'a>(
    scope: &mut HandleScope<'s>,
    function: Local<'a, Function>,
    this: Local<'a, Value>,
) -> Result<Local<'a, Function>, String>
where
    's: 'a,
{
    let bind = get_function(scope, function.into(), "bind")?;
    Ok(bind
        .call(scope, function.into(), &[this])
        .unwrap()
        .try_into()
        .unwrap())
}

pub trait LocalArray {
    fn array_update(&self, scope: &mut HandleScope, new_data: Local<Array>);
    fn clear(&self, scope: &mut HandleScope);
    fn collection_load(&self, scope: &mut HandleScope, collection: String);
    fn copy(&self, scope: &mut HandleScope, origin_array: Local<Array>);
}


impl <'a> LocalArray for Local<'a, Array> {
    fn array_update(&self, scope: &mut HandleScope, new_data: Local<Array>) {
        let _ = &self.clear(scope);

        for index in 0..new_data.length() {
            let item = new_data.get_index(scope, index).unwrap();
            let _ = &self.set_index(scope, index, item);
        }
    }

    fn clear(&self, scope: &mut HandleScope) {
        let length = v8::String::new(scope, "length").unwrap();
        let value = Integer::new(scope, 0);
        let _ = &self.set(scope, length.into(), value.into());
    }

    fn collection_load(&self, scope: &mut HandleScope, collection: String) {
        REPOSITORY.get_all(collection, |item| {
            let value = v8::String::new(scope, &item).unwrap().into();
            let value = json::parse(scope, value).unwrap().into();
            let _ = &self.set_index(scope, self.length(), value);
        });
    }

    fn copy(&self, scope: &mut HandleScope, origin_array: Local<Array>) {
        for index in 0..origin_array.length() {
            let value = origin_array.get_index(scope, index).unwrap();
            let _ = &self.set_index(scope, index, value);
        }
    }
}
