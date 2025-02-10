use std::fs;

use v8::{
    new_default_platform, Context, ContextScope, FunctionCallbackArguments, FunctionTemplate, HandleScope, Isolate, ObjectTemplate, ReturnValue, Script
};

pub struct QueryEngine {
}

impl QueryEngine {
    pub fn new() -> Self {
        let platform = new_default_platform(0, false).make_shared();

        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let isolate = &mut Isolate::new(Default::default());
        let mut handle_scope = HandleScope::new(isolate);

        let global = ObjectTemplate::new(&mut handle_scope);
        
        global.set(
            v8::String::new(&mut handle_scope, "filter").unwrap().into(),
            FunctionTemplate::new(&mut handle_scope, QueryEngine::filter).into(),
        );

        let context = Context::new(&mut handle_scope,
            v8::ContextOptions {
              global_template: Some(global),
              ..Default::default()
            },);

        let scope = &mut ContextScope::new(&mut handle_scope, context);

        let code = fs::read_to_string("./script.js").expect("failed to convert Rust string to javascript string");

        let code = v8::String::new(scope, &code).unwrap();
        println!("javascript code: {}", code.to_rust_string_lossy(scope));

        let script = Script::compile(scope, code, None).unwrap();

        script.run(scope);

        let global = context.global(scope);

        // getting the javascript function whose name is function_name parameter
        let function_name_string = v8::String::new(scope, "name")
            .expect("failed to convert Rust string to javascript string");

        let function = global
            .get(scope, function_name_string.into())
            .expect(&*format!("could not find function {}", "name"));

        let function: v8::Local<v8::Function> = v8::Local::cast(function);
        // call that function with arguments
        let recv = v8::Integer::new(scope, 2).into();

        function.call(scope, recv, &[]);

        Self {
        }
    }

    fn filter(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
        let message = args
            .get(0)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);

        println!("Logged: {}", message);
    }
}
