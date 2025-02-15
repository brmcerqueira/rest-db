use swc_core::bundler::Resolve;
use swc_core::common::FileName;
use swc_core::ecma::loader::resolve::Resolution;

pub struct PathResolve;

impl Resolve for PathResolve {
    fn resolve(&self, base: &FileName, module_specifier: &str) -> Result<Resolution, anyhow::Error> {
        let base = match base {
            FileName::Real(v) => v,
            _ => unreachable!(),
        };

        Ok(Resolution {
            filename: FileName::Real(
                base.parent()
                    .unwrap()
                    .join(module_specifier)
                    .with_extension("ts"),
            ),
            slug: None,
        })
    }
}