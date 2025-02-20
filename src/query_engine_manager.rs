use crate::query_engine::QueryEngine;
use crate::repository::REPOSITORY;
use std::sync::LazyLock;

pub static QUERY_ENGINE_MANAGER: LazyLock<QueryEngineManager> =
    LazyLock::new(|| QueryEngineManager::new());

#[derive(Clone)]
pub struct QueryEngineManager {
    query_engine_production: Option<QueryEngine>,
    query_engine_canary: Option<QueryEngine>,
}

impl QueryEngineManager {
    fn new() -> Self {
        QueryEngineManager {
            query_engine_production: if let Ok(code) = REPOSITORY.script(false) {
                Some(QueryEngine::new(code.clone()))
            } else {
                None
            },
            query_engine_canary: if let Ok(code) = REPOSITORY.script(true) {
                Some(QueryEngine::new(code.clone()))
            } else {
                None
            },
        }
    }

    pub fn refresh(mut self, code: String) {
        self.query_engine_canary = Some(QueryEngine::new(code.clone()));
        REPOSITORY.save_script(code.clone(), true);

        if self.query_engine_production.is_none() {
            self.query_engine_canary = Some(QueryEngine::new(code.clone()));
            REPOSITORY.save_script(code, true);
        }
    }

    pub fn production(self) -> Result<QueryEngine, String> {
        self.query_engine_production
            .ok_or("QueryEngineProduction is missing".to_string())
    }

    pub fn canary(self) -> Option<QueryEngine> {
        self.query_engine_canary
    }

    pub fn promote(mut self) {
        if let Some(query_engine) = self.query_engine_canary {
            self.query_engine_production = Some(query_engine);
        }
    }
}
