use std::sync::LazyLock;

use heed::{Database, Env, EnvOpenOptions};
use heed::types::*;
use serde_json::Value;
use uuid::Uuid;

pub static REPOSITORY: LazyLock<Repository> = LazyLock::new(|| { Repository::new() });

const COLLECTION_KEY: &str = "collection";

pub struct Repository {
    env: Env,
    database: Database<Str, Str>
}

impl Repository {
    fn new() -> Self {
        let env = unsafe { EnvOpenOptions::new().open("db") }.unwrap();

        let mut wtxn = env.write_txn().unwrap();

        Self {
            database: env.create_database(&mut wtxn, None).unwrap(),
            env: env.clone(),
        }
    }

    pub fn get(&self, collection: String, id: String) -> String {
        let mut wtxn = self.env.write_txn().unwrap();
        let key = format!("{COLLECTION_KEY}:{collection}:{id}");
        let data = self.database.get(&mut wtxn, &key).unwrap().unwrap().to_string();
        wtxn.commit().unwrap();
        return data;
    }

    pub fn get_all(&self, collection: String) -> Vec<String> {
        let mut wtxn = self.env.write_txn().unwrap();
        let key = format!("{COLLECTION_KEY}:{collection}");

        let mut vec = Vec::new(); 

        for item in self.database.prefix_iter(&mut wtxn, &key).unwrap() {
            vec.push(item.unwrap().1.to_string());
        }

        wtxn.commit().unwrap();
        return vec;
    }

    pub fn create(&self, collection: String, mut value: Value) -> String {
        let mut wtxn = self.env.write_txn().unwrap();
        let id = Uuid::new_v4().to_string();
        value["$id"] = id.clone().into();
        let data = serde_json::to_string(&value).unwrap();
        let key = format!("{COLLECTION_KEY}:{collection}:{id}");
        self.database.put(&mut wtxn, &key, &data).unwrap();
        wtxn.commit().unwrap();
        return id;
    }
}