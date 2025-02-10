use std::fs;

use v8::{
    Context, ContextScope, FunctionCallbackArguments, FunctionTemplate,
    HandleScope, Local, ObjectTemplate, ReturnValue, Script
};

pub struct QueryEngine<'s, 'i> {
    context: Local<'s, Context>,
    context_scope: ContextScope<'i, HandleScope<'s>>,
}

impl<'s, 'i> QueryEngine<'s, 'i>
where
    's: 'i,
{
    pub fn new(isolate_scope: &'i mut HandleScope<'s, ()>) -> Self {
        let global = ObjectTemplate::new(isolate_scope);

        global.set(
            v8::String::new(isolate_scope, "filter").unwrap().into(),
            FunctionTemplate::new(isolate_scope, QueryEngine::filter).into(),
        );

        let context = Context::new(
            isolate_scope,
            v8::ContextOptions {
                global_template: Some(global),
                ..Default::default()
            },
        );

        let mut context_scope = ContextScope::new(isolate_scope, context);

        let code = fs::read_to_string("./script.js")
            .expect("failed to convert Rust string to javascript string");

        let code = v8::String::new(&mut context_scope, &code).unwrap();

        println!(
            "javascript code: {}",
            code.to_rust_string_lossy(&mut context_scope)
        );

        let script = Script::compile(&mut context_scope, code, None).unwrap();

        script.run(&mut context_scope);

        return QueryEngine {
            context,
            context_scope,
        };
    }

    pub fn call(&mut self, name: &str) {
        let global =  self.context.global(&mut self.context_scope);
  
        // getting the javascript function whose name is function_name parameter
        let function_name_string = v8::String::new(&mut self.context_scope, &name)
            .expect("failed to convert Rust string to javascript string");

        let function = global
            .get(&mut self.context_scope, function_name_string.into())
            .expect(&*format!("could not find function {}", name));

        let function: v8::Local<v8::Function> = v8::Local::cast(function);
        // call that function with arguments
        let recv = v8::Integer::new(&mut self.context_scope, 2).into();

        function.call(&mut self.context_scope, recv, &[]);
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
