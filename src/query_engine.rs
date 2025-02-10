use std::{fs, sync::mpsc::{self, Sender}, thread};

use v8::{
    new_default_platform, Context, ContextOptions, ContextScope, Function, FunctionCallbackArguments, FunctionTemplate, HandleScope, Isolate, Local, ObjectTemplate, ReturnValue, Script, V8::{initialize, initialize_platform}
};

pub struct QueryEngine {
    pub call: Sender<String>
}

impl QueryEngine {
    pub fn new(path: String) -> Self {
        let (call, rx) = mpsc::channel::<String>();
        
        thread::spawn(|| {
            let platform = new_default_platform(0, false).make_shared();

            initialize_platform(platform);
            initialize();
        
            let isolate = &mut Isolate::new(Default::default());
            let mut isolate_scope = HandleScope::new(isolate);
    
            let global_template = ObjectTemplate::new(&mut isolate_scope);
    
            global_template.set(
                v8::String::new(&mut isolate_scope, "filter").unwrap().into(),
                FunctionTemplate::new(&mut isolate_scope, QueryEngine::filter).into(),
            );
    
            let context = Context::new(
                &mut isolate_scope,
                ContextOptions {
                    global_template: Some(global_template),
                    ..Default::default()
                },
            );
    
            let mut context_scope = ContextScope::new(&mut isolate_scope, context);
    
            let code = fs::read_to_string(path)
                .expect("failed to read script path");
    
            let code = v8::String::new(&mut context_scope, &code).unwrap();
    
            let script = Script::compile(&mut context_scope, code, None).unwrap();
    
            script.run(&mut context_scope);
    
            let global = context.global(&mut context_scope);
    
            for name in rx {
                let function_name_string = v8::String::new(&mut context_scope, &name)
                    .expect("failed to convert Rust string to javascript string");
        
                let function = global
                    .get(&mut context_scope, function_name_string.into())
                    .expect(&*format!("could not find function {}", name));
        
                let function: Local<Function> = Local::cast(function);
              
                let recv = v8::Integer::new(&mut context_scope, 2).into();
        
                function.call(&mut context_scope, recv, &[]);
            }  
        });

        return QueryEngine {
            call
        };
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
