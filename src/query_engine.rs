use std::sync::Mutex;
use std::{collections::HashMap, sync::{
    mpsc::{self, Sender},
    LazyLock,
}, thread};
use v8::{json, Array, Boolean, Context, ContextOptions, ContextScope, HandleScope, Integer, Isolate, Local, Number, Object, ObjectTemplate, Script, TryCatch, Value};

use crate::repository::REPOSITORY;
use crate::{
    stages::global_functions, utils::get_function,
};

pub static QUERY_ENGINE: LazyLock<Mutex<QueryEngine>> = LazyLock::new(|| Mutex::new(QueryEngine::new(REPOSITORY.script())));

pub fn refresh_query_engine(code: String) {
    let mut lock = QUERY_ENGINE.lock().unwrap();
    *lock = QueryEngine::new(code);
}


pub struct QueryEngineCall {
    pub name: String,
    pub args: HashMap<String, String>,
    pub result: Sender<String>,
}

pub struct QueryEngine {
    pub call: Sender<QueryEngineCall>,
}

impl QueryEngine {
    fn new(code: String) -> Self {
        print!("Running: {}", code);

        let (call, receiver) = mpsc::channel::<QueryEngineCall>();

        thread::spawn(move || -> Result<(), String> {
            let isolate = &mut Isolate::new(Default::default());

            let isolate_scope = &mut HandleScope::new(isolate);

            let global_template = ObjectTemplate::new(isolate_scope);

            global_functions(isolate_scope, global_template);

            let context = Context::new(
                isolate_scope,
                ContextOptions {
                    global_template: Some(global_template),
                    ..Default::default()
                },
            );

            let context_scope = &mut ContextScope::new(isolate_scope, context);

            let code = v8::String::new(context_scope, &code).unwrap();

            let global = Script::compile(context_scope, code, None)
                .unwrap()
                .run(context_scope).unwrap().to_object(context_scope).unwrap();

            for item in receiver {
                let args = Object::new(context_scope);

                for (key, value) in item.args.iter() {
                    let local_key = v8::String::new(context_scope, key).unwrap();
                    let local_value = Self::parse(context_scope, value);
                    args.set(context_scope, local_key.into(), local_value);
                }

                let array = Array::new(context_scope, 0).into();

                let function = get_function(context_scope, global, &item.name);

                if let Err(err) = function {
                    item.result.send(err.to_string()).unwrap();
                }
                else {
                    let try_catch = &mut TryCatch::new(context_scope);

                    function.unwrap().call(
                        try_catch,
                        array,
                        &[args.into()],
                    );

                    if try_catch.has_caught() {
                        let exception = try_catch.exception().unwrap();
                        let message = exception.to_string(try_catch).unwrap();
                        item.result.send(message.to_rust_string_lossy(try_catch)).unwrap();
                    } else {
                        item.result
                            .send(
                                json::stringify(try_catch, array)
                                    .unwrap()
                                    .to_rust_string_lossy(try_catch),
                            )
                            .unwrap();
                    }
                }
            }
            Ok(())
        });

        QueryEngine { call }
    }

    fn parse<'s>(scope: &mut HandleScope<'s, ()>, input: &String) -> Local<'s, Value> {
        if let Ok(val) = input.parse::<f64>() {
            return Number::new(scope, val).into();
        }

        if let Ok(val) = input.parse::<i32>() {
            return Integer::new(scope, val).into();
        }

        if let "true" = input.to_lowercase().as_str() {
            return Boolean::new(scope, true).into();
        }

        if let "false" = input.to_lowercase().as_str() {
            return Boolean::new(scope, false).into();
        }

        v8::String::new(scope, input).unwrap().into()
    }
}
