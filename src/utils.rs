use v8::{
    Array, Exception, Function, FunctionCallbackArguments, HandleScope, Local, Object, Value,
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

pub fn try_or_throw(
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
