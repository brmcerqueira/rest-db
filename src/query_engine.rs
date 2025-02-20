use std::sync::mpsc::Sender;
use std::sync::{mpsc, Mutex};
use std::{collections::HashMap, sync::LazyLock, thread};
use v8::{
    json, Array, Boolean, Context, ContextOptions, ContextScope, HandleScope, Integer, Isolate,
    Local, Number, Object, ObjectTemplate, Script, TryCatch, Value,
};

use crate::repository::REPOSITORY;
use crate::{stages::global_functions, utils::get_function};

pub static QUERY_ENGINE: LazyLock<Mutex<QueryEngine>> =
    LazyLock::new(|| Mutex::new(QueryEngine::new(REPOSITORY.script())));

pub fn refresh_query_engine(code: String) {
    let mut lock = QUERY_ENGINE.lock().unwrap();
    *lock = QueryEngine::new(code);
}

#[derive(Clone)]
pub struct QueryEngine {
    sender: Sender<QueryEngineCall>,
}

struct QueryEngineCall {
    pub name: String,
    pub args: HashMap<String, String>,
    pub result: Sender<Result<String, String>>,
}

impl QueryEngine {
    fn new(code: String) -> Self {
        print!("Running: {}", code);

        let (sender, receiver) = mpsc::channel::<QueryEngineCall>();

        thread::spawn(move || {
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

            let object = Script::compile(context_scope, code, None)
                .unwrap()
                .run(context_scope)
                .unwrap()
                .to_object(context_scope)
                .unwrap();

            for item in receiver {
                item.result
                    .send(Self::treat_call(
                        context_scope,
                        object,
                        item.name,
                        item.args,
                    ))
                    .unwrap();
            }
        });

        QueryEngine { sender }
    }

    fn treat_call(
        context_scope: &mut ContextScope<HandleScope>,
        object: Local<Object>,
        name: String,
        args: HashMap<String, String>,
    ) -> Result<String, String> {
        let try_catch = &mut TryCatch::new(context_scope);

        let arguments = Object::new(try_catch);

        for (key, value) in args.iter() {
            let local_key = v8::String::new(try_catch, key)
                .ok_or(format!("can't create argument in {name}"))?;
            let local_value = Self::parse(try_catch, value);
            arguments.set(try_catch, local_key.into(), local_value);
        }

        let out = Array::new(try_catch, 0).into();

        let function = get_function(try_catch, object, &name)?;

        function.call(try_catch, out, &[arguments.into()]);

        if try_catch.has_caught() {
            let exception = try_catch
                .exception()
                .ok_or(format!("can't get exception in {name}"))?;
            let message = exception
                .to_string(try_catch)
                .ok_or(format!("can't convert exception to string in {name}"))?;
            Err(format!(
                "Error -> {}",
                message.to_rust_string_lossy(try_catch)
            ))
        } else {
            Ok(json::stringify(try_catch, out)
                .ok_or(format!("can't stringify out in {name}"))?
                .to_rust_string_lossy(try_catch))
        }
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

    pub fn call(self, name: String, args: HashMap<String, String>) -> Result<String, String> {
        let (result, receiver) = mpsc::channel::<Result<String, String>>();

        self.sender
            .send(QueryEngineCall { name, args, result })
            .map_err(|e| e.to_string())?;

        receiver.recv().map_err(|e| e.to_string())?
    }
}
