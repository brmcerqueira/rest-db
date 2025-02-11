mod query_engine;
mod repository;
mod stages;

use std::collections::HashMap;

use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder};
use query_engine::{QueryEngine, QueryEngineCall};
use repository::REPOSITORY;
use serde::{Deserialize, Serialize};
use serde_json::Value;

struct AppState {
    query_engine: QueryEngine
}

#[derive(Deserialize, Serialize)]
struct CollectionCreate {
    id: u64,
}

#[get("/collection/{name}/{id}")]
async fn collection_get(
    path: web::Path<(String, u64)>
) -> impl Responder {
    let (name, id) = path.into_inner();
    let body = REPOSITORY.get(name, id);
    return HttpResponse::Ok().body(body);
}

#[put("/collection/{name}")]
async fn collection_create(
    json: web::Json<Value>,
    path: web::Path<String>
) -> impl Responder {
    let id = REPOSITORY.create(path.into_inner(), json.into_inner());
    return HttpResponse::Ok().json(CollectionCreate { id });
}

#[get("/query/{name}")]
async fn query(
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
    data: web::Data<AppState>,
) -> impl Responder {
    data.query_engine.call.send(QueryEngineCall {
        name: path.into_inner(),
        args: query.0
    }).unwrap();
    return HttpResponse::Ok();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        query_engine: QueryEngine::new("./script.js".to_string())
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(collection_get)
            .service(collection_create)
            .service(query)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
