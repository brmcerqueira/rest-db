use crate::try_catch_verify::TryCatchVerify;
use v8::{
    Array, DataError, Function, FunctionCallbackArguments, HandleScope, Local, Object, TryCatch,
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
        scope.throw_exception(value.into());
    }
}

pub fn out_calculate_operator(
    root_scope: &mut HandleScope,
    args: &FunctionCallbackArguments,
    update: fn(&mut f64, f64),
) -> f64 {
    let mut result = 0f64;

    try_or_throw(root_scope, |scope| {
        let try_catch = &mut TryCatch::new(scope);

        let out = out_array(&args)?;

        let function: Local<Function> = args
            .get(0)
            .try_into()
            .map_err(|x: DataError| x.to_string())?;

        for index in 0..out.length() {
            let item = out
                .get_index(try_catch, index)
                .ok_or("can't get item in calculate")?;

            let call = function.call(try_catch, out.into(), &[item]);

            try_catch.verify()?;

            let value = call
                .ok_or("can't get value from item in calculate")?
                .to_number(try_catch)
                .ok_or("can't convert local number from value in calculate")?
                .number_value(try_catch)
                .ok_or("can't convert number from value in calculate")?;

            update(&mut result, value);
        }

        Ok(())
    });

    result
}
