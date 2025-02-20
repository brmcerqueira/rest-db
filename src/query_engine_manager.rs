use crate::query_engine::QueryEngine;
use crate::repository::REPOSITORY;
use std::sync::{LazyLock, Mutex};

static QUERY_ENGINE_PRODUCTION: LazyLock<Mutex<Option<QueryEngine>>> = LazyLock::new(|| {
    Mutex::new(if let Ok(code) = REPOSITORY.script(false) {
        Some(QueryEngine::new(code))
    } else {
        None
    })
});

static QUERY_ENGINE_CANARY: LazyLock<Mutex<Option<QueryEngine>>> = LazyLock::new(|| {
    Mutex::new(if let Ok(code) = REPOSITORY.script(true) {
        Some(QueryEngine::new(code))
    } else {
        None
    })
});

pub fn refresh(code: String) {
    let mut mutex = QUERY_ENGINE_CANARY.lock().unwrap();
    *mutex = Some(QueryEngine::new(code.clone()));
    REPOSITORY.save_script(code.clone(), true);

    let mut mutex = QUERY_ENGINE_PRODUCTION.lock().unwrap();

    if mutex.is_some() {
        *mutex = Some(QueryEngine::new(code.clone()));
        REPOSITORY.save_script(code, false);
    }
}

pub fn production() -> Result<QueryEngine, String> {
    QUERY_ENGINE_PRODUCTION
        .lock()
        .unwrap()
        .clone()
        .ok_or("QueryEngineProduction is missing".to_string())
}

pub fn canary() -> Result<QueryEngine, String> {
    QUERY_ENGINE_CANARY
        .lock()
        .unwrap()
        .clone()
        .ok_or("QueryEngineCanary is missing".to_string())
}

pub fn promote() {
    if let Some(query_engine) = QUERY_ENGINE_CANARY.lock().unwrap().take() {
        let mut mutex = QUERY_ENGINE_PRODUCTION.lock().unwrap();
        *mutex = Some(query_engine);
    }
}
