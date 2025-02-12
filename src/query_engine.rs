use regex::Regex;
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
    new_default_platform, Context, ContextOptions, ContextScope, Function, HandleScope, Isolate,
    Local, ObjectTemplate, Script,
    V8::{initialize, initialize_platform},
};

use crate::stages::global_functions;

static FUNCTION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(function)?\s*\$[\w]+\s*\(.*?\)").unwrap());
static FUNCTION_CALL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\s*\$[\w]+)\s*\((.*?)\)").unwrap());
pub static QUERY_ENGINE: LazyLock<QueryEngine> = LazyLock::new(|| QueryEngine::new());

pub struct QueryEngineCall {
    pub name: String,
    pub args: HashMap<String, String>,
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

            let code = QueryEngine::sanitize(
                fs::read_to_string(&path).expect(&*format!("could not find script {path}")),
            );

            println!("Code: {}", code);

            let code = v8::String::new(&mut context_scope, &code).unwrap();

            let script = Script::compile(&mut context_scope, code, None).unwrap();

            script.run(&mut context_scope);

            let global = context.global(&mut context_scope);

            for item in receiver {
                let function_name_string = v8::String::new(&mut context_scope, &item.name)
                    .expect("failed to convert Rust string to javascript string");

                let function = global
                    .get(&mut context_scope, function_name_string.into())
                    .expect(&*format!("could not find function {}", &item.name));

                let function: Local<Function> = function.try_into().unwrap();

                let args = v8::Object::new(&mut context_scope);

                for (key, value) in item.args.iter() {
                    let local_key = v8::String::new(&mut context_scope, key).unwrap();
                    let local_value = QueryEngine::parse(&mut context_scope, value);
                    args.set(&mut context_scope, local_key.into(), local_value);
                }

                let recv = v8::Array::new(&mut context_scope, 0).into();

                function.call(&mut context_scope, recv, &[args.into()]);
            }
        });

        return QueryEngine { call };
    }

    fn parse<'s>(scope: &mut HandleScope<'s, ()>, input: &String) -> v8::Local<'s, v8::Value> {
        if let Ok(val) = input.parse::<f64>() {
            return v8::Number::new(scope, val).into();
        }

        if let Ok(val) = input.parse::<i32>() {
            return v8::Integer::new(scope, val).into();
        }

        if let "true" = input.to_lowercase().as_str() {
            return v8::Boolean::new(scope, true).into();
        }

        if let "false" = input.to_lowercase().as_str() {
            return v8::Boolean::new(scope, false).into();
        }

        return v8::String::new(scope, input).unwrap().into();
    }

    fn sanitize(code: String) -> String {
        return FUNCTION_REGEX.replace_all(&code, |caps: &regex::Captures| {
            if caps[0].starts_with("function") {
                return caps[0].to_string();
            }
    
            return FUNCTION_CALL_REGEX.replace(&caps[0], r"$1.call(this, $2)")
            .to_string();
        }).to_string();
    }
}
