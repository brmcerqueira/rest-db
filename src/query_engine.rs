use regex::{Captures, Regex};
use std::{
    collections::HashMap,
    fs,
    sync::{
        mpsc::{self, Sender},
        LazyLock,
    },
    thread,
};

use v8::{
    json, new_default_platform, Array, Boolean, Context, ContextOptions, ContextScope, HandleScope,
    Integer, Isolate, Local, Number, Object, ObjectTemplate, Script, Value,
    V8::{initialize, initialize_platform},
};

use crate::{stages::global_functions, utils::get_function};

static FUNCTION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(function)?\s*\$[\w]+\s*\(.*?\)").unwrap());
static FUNCTION_CALL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\s*\$[\w]+)\s*\((.*?)\)").unwrap());
pub static QUERY_ENGINE: LazyLock<QueryEngine> = LazyLock::new(|| QueryEngine::new());

pub struct QueryEngineCall {
    pub name: String,
    pub args: HashMap<String, String>,
    pub result: Sender<String>,
}

pub struct QueryEngine {
    pub call: Sender<QueryEngineCall>,
}

impl QueryEngine {
    fn new() -> Self {
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

            let path = "./script.js";

            let code = v8::String::new(&mut context_scope, &QueryEngine::sanitize(
                fs::read_to_string(&path).expect(&*format!("could not find script {path}")),
            )).unwrap();

            Script::compile(&mut context_scope, code, None).unwrap().run(&mut context_scope);

            let global = context.global(&mut context_scope);

            for item in receiver {
                let args = Object::new(&mut context_scope);

                for (key, value) in item.args.iter() {
                    let local_key = v8::String::new(&mut context_scope, key).unwrap();
                    let local_value = QueryEngine::parse(&mut context_scope, value);
                    args.set(&mut context_scope, local_key.into(), local_value);
                }

                let array = Array::new(&mut context_scope, 0).into();

                get_function(&mut context_scope, global.into(), &item.name).call(
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

        return QueryEngine { call };
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

        return v8::String::new(scope, input).unwrap().into();
    }

    fn sanitize(code: String) -> String {
        return FUNCTION_REGEX
            .replace_all(&code, |caps: &Captures| {
                if caps[0].starts_with("function") {
                    return caps[0].to_string();
                }

                return FUNCTION_CALL_REGEX
                    .replace(&caps[0], r"$1.call(this, $2)")
                    .to_string();
            })
            .to_string();
    }
}
