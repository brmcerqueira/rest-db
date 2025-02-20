use v8::{
    Array, DataError, Exception, Function, FunctionCallbackArguments, HandleScope, Local, Object
    ,
};

pub fn get_function<'s, 'a>(
    scope: &mut HandleScope<'s>,
    object: Local<'a, Object>,
    name: &str,
) -> Result<Local<'a, Function>, String>
where
    's: 'a,
{
    let function_name =
        v8::String::new(scope, name).ok_or(format!("can't create string {name}"))?;

    let function = object
        .get(scope, function_name.into())
        .map(|value| {
            if value.is_undefined() {
                Err(format!("could not find function {name}"))
            } else {
                Ok(value)
            }
        })
        .unwrap();

    Ok(function?.try_into().map_err(|x: DataError| x.to_string())?)
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
