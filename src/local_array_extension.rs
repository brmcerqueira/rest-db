use crate::repository::REPOSITORY;
use v8::{json, Array, HandleScope, Integer, Local};

pub trait LocalArrayExtension {
    fn clear(&self, scope: &mut HandleScope);
    fn set_length(&self, scope: &mut HandleScope, value: i32);
    fn collection_load(&self, scope: &mut HandleScope, collection: String);
    fn copy(&self, scope: &mut HandleScope, origin_array: Local<Array>);
}

impl<'a> LocalArrayExtension for Local<'a, Array> {
    fn clear(&self, scope: &mut HandleScope) {
        self.set_length(scope, 0);
    }

    fn set_length(&self, scope: &mut HandleScope, value: i32) {
        let length = v8::String::new(scope, "length").unwrap();
        let value = Integer::new(scope, value);
        let _ = &self.set(scope, length.into(), value.into());
    }

    fn collection_load(&self, scope: &mut HandleScope, collection: String) {
        REPOSITORY.get_all(collection, |item| {
            let value = v8::String::new(scope, &item).unwrap().into();
            let value = json::parse(scope, value).unwrap().into();
            let _ = &self.set_index(scope, self.length(), value);
        });
    }

    fn copy(&self, scope: &mut HandleScope, origin_array: Local<Array>) {
        for index in 0..origin_array.length() {
            let value = origin_array.get_index(scope, index).unwrap();
            let _ = &self.set_index(scope, index, value);
        }
    }
}
