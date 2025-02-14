use regex::{Captures, Regex};
use std::{
    collections::HashMap,
    path::Path,
    sync::{
        mpsc::{self, Sender},
        LazyLock,
    },
    thread,
};
use swc_core::{
    common::{
        comments::SingleThreadedComments,
        errors::{ColorConfig, Handler},
        sync::Lrc,
        Globals, Mark, SourceMap, GLOBALS,
    },
    ecma::{
        codegen::{text_writer::JsWriter, Emitter},
        parser::{lexer::Lexer, Parser, StringInput, Syntax},
        transforms::typescript::strip,
        visit::{swc_ecma_ast::{CallExpr, Callee, Expr, ExprOrSpread, Ident, MemberExpr, MemberProp}, Fold, FoldWith},
    },
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

struct CallTransformer;

impl Fold for CallTransformer {
    fn fold_call_expr(&mut self, call: CallExpr) -> CallExpr {
        return CallExpr {
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: call.span,
                obj: Box::new(Expr::Ident(Ident {
                    span: call.span,
                    sym: "this".into(),
                    optional: false,
                })),
                prop: MemberProp::Ident(Ident {
                    span: call.span,
                    sym: "apply".into(),
                    optional: false,
                }),
            }))),
            /*args: vec![
                ExprOrSpread {
                    spread: None,
                    expr: call.callee,
                },
                ExprOrSpread {
                    spread: None,
                    expr: call.args.into(),
                },
            ],*/
            ..call
        };
    }
}

impl QueryEngine {
    fn new() -> Self {
        let (call, receiver) = mpsc::channel::<QueryEngineCall>();

        let path = "./script.ts";

        let cm: Lrc<SourceMap> = Default::default();
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

        let source = cm
            .load_file(Path::new(&path))
            .expect("failed to load input typescript file");

        let comments = SingleThreadedComments::default();

        let lexer = Lexer::new(
            Syntax::Typescript(Default::default()),
            Default::default(),
            StringInput::from(&*source),
            Some(&comments),
        );

        let mut parser = Parser::new_from(lexer);

        for e in parser.take_errors() {
            e.into_diagnostic(&handler).emit();
        }

        let program = parser
            .parse_program()
            .map_err(|e| e.into_diagnostic(&handler).emit())
            .expect("failed to parse program.");

        let globals = Globals::default();

        let code = GLOBALS.set(&globals, || {
            let top_level_mark = Mark::new();

            let program = program.fold_with(&mut strip(top_level_mark)).fold_with(&mut CallTransformer);

            let mut buf = vec![];

            let mut emitter = Emitter {
                cfg: swc_core::ecma::codegen::Config::default(),
                cm: cm.clone(),
                comments: Some(&comments),
                wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
            };

            emitter.emit_program(&program).unwrap();

            let code = String::from_utf8(buf).expect("non-utf8?");

            println!("Code: {}", code);

            return code;
        });

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

            let code = v8::String::new(&mut context_scope, &Self::sanitize(code)).unwrap();

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
