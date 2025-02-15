use crate::typescript::path_resolve::PathResolve;
use crate::typescript::ts_load::TsLoad;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use swc_core::bundler::{Bundler, Hook, ModuleRecord, ModuleType};
use swc_core::common::{FileLoader, FileName, FilePathMapping, Span};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::swc_ecma_ast::KeyValueProp;
use swc_core::{bundler, common::{
    Globals, SourceMap,
}, ecma::codegen::{text_writer::JsWriter, Emitter}};
use virtual_filesystem::FileSystem;
use virtual_filesystem::zip_fs::ZipFS;

struct Noop;

impl Hook for Noop {
    fn get_import_meta_props(&self, _: Span, _: &ModuleRecord) -> Result<Vec<KeyValueProp>, anyhow::Error> {
        unimplemented!()
    }
}

pub struct TsTranspiler {
    pub code: String,
}

pub struct TsFileLoader<R: Read + Seek> {
    zip_fs: ZipFS<R>,
    root: PathBuf
}

impl <R: Read + Seek> FileLoader for TsFileLoader<R> {
    fn file_exists(&self, path: &Path) -> bool {
        self.zip_fs.exists(self.root.join(path).to_str().unwrap()).unwrap()
    }

    fn abs_path(&self, path: &Path) -> Option<PathBuf> {
        Some(path.to_path_buf())
    }

    fn read_file(&self, path: &Path) -> io::Result<String> {
        match self.zip_fs.open_file(self.root.join(path).to_str().unwrap()) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                Ok(content)
            },
            _ => Ok(String::new()),
        }
    }
}

impl TsTranspiler {
    pub fn new<R: Read + Seek + Send + 'static>(reader: R, main: String) -> Self {
        let mut entries = HashMap::default();

        let zip_fs = ZipFS::new(reader).unwrap();

        let root = "/test_data";

        for dir in zip_fs.read_dir(root).unwrap().flatten() {
            let path = dir.path;
            if path.ends_with(&format!("{}.ts", main).to_string()) {
                entries.insert(path.to_str().unwrap().to_string(), FileName::Real(path));
            }
        }

        let cm: Rc<SourceMap> = Rc::new(SourceMap::with_file_loader(Box::new(TsFileLoader::<R> {
            zip_fs,
            root: Path::new(root).to_path_buf()
        }), FilePathMapping::default()));

        let globals = Globals::default();

        let mut bundler = Bundler::new(
            &globals,
            cm.clone(),
            TsLoad { cm: cm.clone() },
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
            cfg: codegen::Config::default(),
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
        };

        emitter.emit_module(&bundles.pop().unwrap().module).unwrap();

        let code = String::from_utf8(buf).unwrap();

        println!("code: {}", code);

        Self { code }
    }
}