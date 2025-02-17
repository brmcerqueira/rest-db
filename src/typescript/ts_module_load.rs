use crate::typescript::query_engine_transformer::QueryEngineTransformer;
use std::rc::Rc;
use swc_core::bundler::{Load, ModuleData};
use swc_core::common::errors::{ColorConfig, Handler};
use swc_core::common::input::StringInput;
use swc_core::common::{FileName, Mark, SourceMap};
use swc_core::ecma::parser::lexer::Lexer;
use swc_core::ecma::parser::{Parser, Syntax};
use swc_core::ecma::transforms::base::fixer::fixer;
use swc_core::ecma::transforms::base::hygiene::hygiene;
use swc_core::ecma::transforms::base::resolver;
use swc_core::ecma::transforms::typescript::strip;
use swc_core::ecma::visit::swc_ecma_ast::{EsVersion, Program};
use swc_core::ecma::visit::VisitMutWith;

pub struct TsModuleLoad {
    pub cm: Rc<SourceMap>,
}

impl Load for TsModuleLoad {
    fn load(&self, file_name: &FileName) -> Result<ModuleData, anyhow::Error> {
        let fm = match file_name {
            FileName::Real(path) => self.cm.load_file(path)?,
            _ => unreachable!(),
        };

        let module = Parser::new_from(Lexer::new(
            Syntax::Typescript(Default::default()),
            EsVersion::latest(),
            StringInput::from(&*fm),
            None,
        )).parse_typescript_module()
            .map_err(|err| {
                let handler = Handler::with_tty_emitter(ColorConfig::Always, false, false, Some(self.cm.clone()));
                err.into_diagnostic(&handler).emit();
            })
            .map(|module| {
                let unresolved_mark = Mark::new();
                let top_level_mark = Mark::new();

                let mut module = Program::Module(module)
                    .apply(resolver(unresolved_mark, top_level_mark, true))
                    .apply(strip(unresolved_mark, top_level_mark))
                    .apply(hygiene())
                    .apply(fixer(None))
                    .expect_module();

                module.visit_mut_with(&mut QueryEngineTransformer);

                module
            })
            .unwrap();

        Ok(ModuleData {
            fm,
            module,
            helpers: Default::default(),
        })
    }
}