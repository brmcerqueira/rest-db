use std::time::{SystemTime, UNIX_EPOCH};

use heed::{Database, Env, EnvOpenOptions};
use heed::types::*;
use serde_json::Value;

const COLLECTION_KEY: &str = "collection";

pub struct Repository {
    env: Env,
    database: Database<Str, Str>
}

impl Repository {
    pub fn new() -> Self {
        let env = unsafe { EnvOpenOptions::new().open("db") }.unwrap();

        let mut wtxn = env.write_txn().unwrap();

        Self {
            database: env.create_database(&mut wtxn, None).unwrap(),
            env: env.clone(),
        }
    }

    fn generate_id() -> u64 {
        let start = SystemTime::now();
        let duration = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        duration.as_secs()
    }

    pub fn get(&self, collection: String, id: u64) -> String {
        let mut wtxn = self.env.write_txn().unwrap();
        let key = format!("{COLLECTION_KEY}:{collection}:{id}");
        let data = self.database.get(&mut wtxn, &key).unwrap().unwrap().to_string();
        wtxn.commit();
        return data;
    }

    pub fn create(&self, collection: String, mut value: Value) -> u64 {
        let mut wtxn = self.env.write_txn().unwrap();
        let id = Repository::generate_id();
        value["$id"] = id.into();
        let data = serde_json::to_string(&value).unwrap();
        let key = format!("{COLLECTION_KEY}:{collection}:{id}");
        self.database.put(&mut wtxn, &key, &data);
        wtxn.commit();
        return id;
    }
}