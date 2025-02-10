use v8::{
    new_default_platform, Context, ContextScope, FunctionCallbackArguments, FunctionTemplate,
    HandleScope, Isolate, ObjectTemplate, ReturnValue, Script,
};

pub struct QueryEngine {}

impl QueryEngine {
    pub fn new() -> Self {
        let platform = new_default_platform(0, false).make_shared();

        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        Self {}
    }

    fn filter(scope: &mut HandleScope, args: FunctionCallbackArguments, mut _retval: ReturnValue) {
        let message = args
            .get(0)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);

        println!("Logged: {}", message);
    }

    pub fn run(&mut self, code: String) {
        let isolate = &mut Isolate::new(Default::default());
        let mut handle_scope = HandleScope::new(isolate);
        let context = Context::new(&mut handle_scope, Default::default());
        let scope = &mut ContextScope::new(&mut handle_scope, context);

        let global = ObjectTemplate::new(scope);

        global.set(
            v8::String::new(scope, "filter").unwrap().into(),
            FunctionTemplate::new(scope, QueryEngine::filter).into(),
        );

        let code = v8::String::new(scope, &code).unwrap();
        println!("javascript code: {}", code.to_rust_string_lossy(scope));

        let script = Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();
        let result = result.to_string(scope).unwrap();
        println!("result: {}", result.to_rust_string_lossy(scope));

        let global = context.global(scope);

        // getting the javascript function whose name is function_name parameter
        let function_name_string = v8::String::new(scope, "function_name")
            .expect("failed to convert Rust string to javascript string");

        let function = global
            .get(scope, function_name_string.into())
            .expect(&*format!("could not find function {}", "function_name"));

        let function: v8::Local<v8::Function> = unsafe { v8::Local::cast(function) };
        // call that function with arguments
        let recv = v8::Integer::new(scope, 2).into();

        function.call(scope, recv, &[]);
    }
}
