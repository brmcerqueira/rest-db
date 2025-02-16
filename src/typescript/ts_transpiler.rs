use crate::typescript::path_resolve::PathResolve;
use crate::typescript::ts_module_load::TsModuleLoad;
use std::collections::HashMap;
use std::io::{Read, Seek};
use std::path::Path;
use std::rc::Rc;
use swc_core::bundler::{Bundler, Hook, ModuleRecord, ModuleType};
use swc_core::common::{FileName, FilePathMapping, Span};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::swc_ecma_ast::KeyValueProp;
use swc_core::{bundler, common::{
    Globals, SourceMap,
}, ecma::codegen::{text_writer::JsWriter, Emitter}};
use virtual_filesystem::FileSystem;
use virtual_filesystem::zip_fs::ZipFS;
use crate::query_engine::refresh_query_engine;
use crate::repository::REPOSITORY;
use crate::typescript::ts_file_loader::TsFileLoader;

struct Noop;

impl Hook for Noop {
    fn get_import_meta_props(&self, _: Span, _: &ModuleRecord) -> Result<Vec<KeyValueProp>, anyhow::Error> {
        unimplemented!()
    }
}

pub fn ts_transpiler<R: Read + Seek + Send + 'static>(reader: R, main: String) {
    let mut entries = HashMap::default();

    let zip_fs = ZipFS::new(reader).unwrap();

    let mut root = String::from(".");

    let mut next = root.clone();

    for _ in 0..2 {
        let dirs = zip_fs.read_dir(next.as_str()).unwrap().flatten();

        for dir in dirs {
            let path = dir.path.clone();
            if path.ends_with(&format!("{}.ts", main).to_string()) {
                entries.insert(path.to_str().unwrap().to_string(), FileName::Real(path));
                break;
            }
            else {
                next = dir.path.to_str().unwrap().to_string();
            }
        }

        if entries.len() == 1 {
            break;
        }
        else {
            root = next.clone();
        }
    }

    let cm: Rc<SourceMap> = Rc::new(SourceMap::with_file_loader(Box::new(TsFileLoader::<R>::new(
        zip_fs,
        Path::new(&root).to_path_buf()
    )), FilePathMapping::default()));

    let globals = Globals::default();

    let mut bundler = Bundler::new(
        &globals,
        cm.clone(),
        TsModuleLoad { cm: cm.clone() },
        PathResolve { cm: cm.clone() },
        bundler::Config {
            require: false,
            disable_inliner: true,
            external_modules: Default::default(),
            disable_fixer: false,
            disable_hygiene: false,
            disable_dce: false,
            module: ModuleType::Iife,
        },
        Box::new(Noop),
    );

    let mut bundles = bundler.bundle(entries).unwrap();

    let mut buf = vec![];

    let mut emitter = Emitter {
        cfg: codegen::Config::default().with_minify(false),
        cm: cm.clone(),
        comments: None,
        wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
    };

    emitter.emit_module(&bundles.pop().unwrap().module).unwrap();

    let code = String::from_utf8(buf).unwrap();

    print!("Transpiled: {}", code);

    refresh_query_engine(code.clone());

    REPOSITORY.save_script(code);
}