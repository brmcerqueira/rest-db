use v8::{HandleScope, TryCatch};

pub trait TryCatchVerify {
    fn verify(&mut self) -> Result<(), String>;
}

impl <'s, 'a> TryCatchVerify for TryCatch<'a, HandleScope<'s>> {
    fn verify(&mut self) -> Result<(), String> {
        if self.has_caught() {
            Err(self.exception().unwrap().to_rust_string_lossy(self))
        }
        else {
            Ok(())
        }
    }
}