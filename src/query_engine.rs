use std::{
    collections::HashMap,
    path::Path,
    rc::Rc,
    sync::{
        mpsc::{self, Sender},
        LazyLock,
    },
    thread,
};
use swc_core::bundler::{Bundler, Hook, Load, ModuleData, ModuleRecord, Resolve};
use swc_core::common::{FileName, Span};
use swc_core::ecma::codegen;
use swc_core::ecma::loader::resolve::Resolution;
use swc_core::ecma::parser::parse_file_as_module;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::visit::swc_ecma_ast::{EsVersion, KeyValueProp};
use swc_core::{bundler, common::{
    comments::SingleThreadedComments,
    errors::{ColorConfig, Handler},
    Globals, Mark, SourceMap, GLOBALS,
}, ecma::{
    codegen::{text_writer::JsWriter, Emitter},
    parser::{lexer::Lexer, Parser, StringInput, Syntax},
    transforms::typescript::strip,
    visit::FoldWith,
}};
use v8::{
    json, new_default_platform, Array, Boolean, Context, ContextOptions, ContextScope, HandleScope,
    Integer, Isolate, Local, Number, Object, ObjectTemplate, Script, Value,
    V8::{initialize, initialize_platform},
};

use crate::{
    call_function_with_context_transformer::CallFunctionWithContextTransformer,
    stages::global_functions, utils::get_function,
};

pub static QUERY_ENGINE: LazyLock<QueryEngine> = LazyLock::new(|| QueryEngine::new());

pub struct QueryEngineCall {
    pub name: String,
    pub args: HashMap<String, String>,
    pub result: Sender<String>,
}

pub struct QueryEngine {
    pub call: Sender<QueryEngineCall>,
}

struct Noop;

impl Hook for Noop {
    fn get_import_meta_props(&self, _: Span, _: &ModuleRecord) -> Result<Vec<KeyValueProp>, anyhow::Error> {
        unimplemented!()
    }
}

pub struct Loader {
    pub cm: Rc<SourceMap>,
}

impl Load for Loader {
    fn load(&self, f: &FileName) -> Result<ModuleData, anyhow::Error> {
        let fm = match f {
            FileName::Real(path) => self.cm.load_file(path)?,
            _ => unreachable!(),
        };

        let module = parse_file_as_module(
            &fm,
            Syntax::Es(Default::default()),
            EsVersion::Es2020,
            None,
            &mut Vec::new(),
        )
            .unwrap_or_else(|err| {
                let handler =
                    Handler::with_tty_emitter(ColorConfig::Always, false, false, Some(self.cm.clone()));
                err.into_diagnostic(&handler).emit();
                panic!("failed to parse")
            });

        Ok(ModuleData {
            fm,
            module,
            helpers: Default::default(),
        })
    }
}

struct PathResolver;

impl Resolve for PathResolver {
    fn resolve(&self, base: &FileName, module_specifier: &str) -> Result<Resolution, anyhow::Error> {
        assert!(
            module_specifier.starts_with('.'),
            "We are not using node_modules within this example"
        );

        let base = match base {
            FileName::Real(v) => v,
            _ => unreachable!(),
        };

        Ok(Resolution {
            filename: FileName::Real(
                base.parent()
                    .unwrap()
                    .join(module_specifier)
                    .with_extension("js"),
            ),
            slug: None,
        })
    }
}

impl QueryEngine {
    fn new() -> Self {
        let (call, receiver) = mpsc::channel::<QueryEngineCall>();

        let path = "./script.ts";

        let cm: Rc<SourceMap> = Default::default();
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

        let source = cm
            .load_file(Path::new(&path))
            .expect("failed to load input typescript file");

        let comments = SingleThreadedComments::default();

        let mut parser = Parser::new_from(Lexer::new(
            Syntax::Typescript(Default::default()),
            Default::default(),
            StringInput::from(&*source),
            Some(&comments),
        ));

        for e in parser.take_errors() {
            e.into_diagnostic(&handler).emit();
        }

        let globals = Globals::default();

        let mut bundler = Bundler::new(
            &globals,
            cm.clone(),
            Loader { cm: cm.clone() },
            PathResolver,
            bundler::Config {
                require: false,
                disable_inliner: true,
                external_modules: Default::default(),
                disable_fixer: false,
                disable_hygiene: false,
                disable_dce: false,
                module: Default::default(),
            },
            Box::new(Noop),
        );

        //let mut entries = HashMap::default();
        //entries.insert("script".to_string(), FileName::Real(path.into()));
        //let mut bundles = bundler.bundle(entries).expect("failed to bundle");

        let code = GLOBALS.set(&globals, || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            let module = parser.parse_program()
            .map_err(|err| err.into_diagnostic(&handler).emit())
            .map(|module| module.apply(resolver(unresolved_mark, top_level_mark, true)))
            .map(|module| module.apply(strip(unresolved_mark, top_level_mark)))
            .map(|module| module.fold_with(&mut CallFunctionWithContextTransformer))
            .unwrap();

            let mut buf = vec![];

            let mut emitter = Emitter {
                cfg: codegen::Config::default(),
                cm: cm.clone(),
                comments: Some(&comments),
                wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
            };

            emitter.emit_program(&module).unwrap();

            //let bundle = bundles.pop().unwrap();

            //emitter.emit_module(&bundle.module).unwrap();

            return String::from_utf8(buf).expect("non-utf8?");
        });

        println!("code: {:?}", code);

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

            Script::compile(&mut context_scope, code, None)
                .unwrap()
                .run(&mut context_scope);

            let global = context.global(&mut context_scope);

            for item in receiver {
                let args = Object::new(&mut context_scope);

                for (key, value) in item.args.iter() {
                    let local_key = v8::String::new(&mut context_scope, key).unwrap();
                    let local_value = Self::parse(&mut context_scope, value);
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
