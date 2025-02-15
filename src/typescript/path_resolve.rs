use std::rc::Rc;
use swc_core::bundler::Resolve;
use swc_core::common::{FileName, SourceMap};
use swc_core::ecma::loader::resolve::Resolution;

pub struct PathResolve {
    pub cm: Rc<SourceMap>,
}

impl Resolve for PathResolve {
    fn resolve(&self, base: &FileName, module_specifier: &str) -> Result<Resolution, anyhow::Error> {
        let base = match base {
            FileName::Real(v) => v,
            _ => unreachable!(),
        };

        let path = base.parent()
            .unwrap()
            .join(module_specifier);

        let path = if self.cm.file_exists(path.with_extension("d.ts").as_path()) {
            path.with_extension("d.ts")
        }
        else {
            path.with_extension("ts")
        };

        Ok(Resolution {
            filename: FileName::Real(path),
            slug: None,
        })
    }
}