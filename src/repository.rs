use std::sync::LazyLock;

use heed::{Database, Env, EnvOpenOptions};
use heed::types::*;
use serde_json::Value;
use uuid::Uuid;

pub static REPOSITORY: LazyLock<Repository> = LazyLock::new(|| { Repository::new() });

const COLLECTION_KEY: &str = "collection";
const SCRIPT_KEY: &str = "script";

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
        let data = self.database.get(&mut wtxn, &key).unwrap().expect("Item not found").to_string();
        wtxn.commit().unwrap();
        data
    }

    pub fn get_all(&self, collection: String, mut each: impl FnMut(String)) {
        let mut wtxn = self.env.write_txn().unwrap();
        let key = format!("{COLLECTION_KEY}:{collection}");

        for item in self.database.prefix_iter(&mut wtxn, &key).unwrap() {
            each(item.unwrap().1.to_string())
        }

        wtxn.commit().unwrap();
    }

    pub fn create(&self, collection: String, mut value: Value) -> String {
        let mut wtxn = self.env.write_txn().unwrap();
        let id = Uuid::new_v4().to_string();
        value["$id"] = id.clone().into();
        let data = serde_json::to_string(&value).unwrap();
        let key = format!("{COLLECTION_KEY}:{collection}:{id}");
        self.database.put(&mut wtxn, &key, &data).unwrap();
        wtxn.commit().unwrap();
        id
    }

    pub fn update(&self, collection: String, id: String, mut value: Value) {
        let mut wtxn = self.env.write_txn().unwrap();
        value["$id"] = id.clone().into();
        let data = serde_json::to_string(&value).unwrap();
        let key = format!("{COLLECTION_KEY}:{collection}:{id}");
        if self.database.get(&mut wtxn, &key).is_ok() {
            self.database.put(&mut wtxn, &key, &data).unwrap(); 
        }
        wtxn.commit().unwrap();
    }

    pub fn delete(&self, collection: String, id: String) {
        let mut wtxn = self.env.write_txn().unwrap();
        let key = format!("{COLLECTION_KEY}:{collection}:{id}");
        self.database.delete(&mut wtxn, &key).unwrap();
        wtxn.commit().unwrap();
    }

    pub fn script(&self) -> String {
        let mut wtxn = self.env.write_txn().unwrap();
        let data = self.database.get(&mut wtxn, SCRIPT_KEY).unwrap().expect("Script not found").to_string();
        wtxn.commit().unwrap();
        data
    }

    pub fn save_script(&self, code: String) {
        let mut wtxn = self.env.write_txn().unwrap();
        self.database.put(&mut wtxn, SCRIPT_KEY, &code).unwrap();
        wtxn.commit().unwrap();
    }
}