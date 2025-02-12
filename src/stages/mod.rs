pub mod filter; 
pub mod collection;

use v8::{FunctionTemplate, HandleScope, Local, ObjectTemplate};

macro_rules! global {
    ($([$name:expr, $cb:expr]),*) => {
        pub fn global_functions<'s>(scope: &mut HandleScope<'s, ()>, global: Local<'s, ObjectTemplate>) {
            $(
                global.set(
                    v8::String::new(scope, $name).unwrap().into(),
                    FunctionTemplate::new(scope, $cb).into(),
                );
            )*
        }
    };
}

global!(
    ["$filter", filter::filter],
    ["$collection", collection::collection]
);