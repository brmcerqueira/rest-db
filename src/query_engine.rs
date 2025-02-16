use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Sender},
        LazyLock,
    },
    thread,
};
use std::string::ToString;
use v8::{
    json, new_default_platform, Array, Boolean, Context, ContextOptions, ContextScope, HandleScope,
    Integer, Isolate, Local, Number, Object, ObjectTemplate, Script, Value,
    V8::{initialize, initialize_platform},
};

use crate::{
    stages::global_functions, utils::get_function,
};

pub static QUERY_ENGINE: LazyLock<QueryEngine> = LazyLock::new(|| QueryEngine::new("".to_string()));

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
        let (call, receiver) = mpsc::channel::<QueryEngineCall>();

        thread::spawn(move || {
            let platform = new_default_platform(0, false).make_shared();

            initialize_platform(platform);

            initialize();

            let isolate = &mut Isolate::new(Default::default());

            let mut isolate_scope = HandleScope::new(isolate);

            let global_template = ObjectTemplate::new(&mut isolate_scope);

            global_functions(&mut isolate_scope, global_template);

            let context = Context::new(
                &mut isolate_scope,
                ContextOptions {
                    global_template: Some(global_template),
                    ..Default::default()
                },
            );

            let mut context_scope = ContextScope::new(&mut isolate_scope, context);

            let code = v8::String::new(&mut context_scope, &code).unwrap();

            let result = Script::compile(&mut context_scope, code, None)
                .unwrap()
                .run(&mut context_scope).unwrap().to_object(&mut context_scope).unwrap();

            for item in receiver {
                let args = Object::new(&mut context_scope);

                for (key, value) in item.args.iter() {
                    let local_key = v8::String::new(&mut context_scope, key).unwrap();
                    let local_value = Self::parse(&mut context_scope, value);
                    args.set(&mut context_scope, local_key.into(), local_value);
                }

                let array = Array::new(&mut context_scope, 0).into();

                get_function(&mut context_scope, result, &item.name).call(
                    &mut context_scope,
                    array,
                    &[args.into()],
                );

                item.result
                    .send(
                        json::stringify(&mut context_scope, array)
                            .unwrap()
                            .to_rust_string_lossy(&mut context_scope),
                    )
                    .unwrap();
            }
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
