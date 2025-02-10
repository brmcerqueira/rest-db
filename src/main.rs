mod query_engine;
mod repository;
use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder};
use query_engine::QueryEngine;
use repository::Repository;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use v8::{new_default_platform, HandleScope, Isolate};

struct AppState {
    repository: Repository,
}

#[derive(Deserialize, Serialize)]
struct CollectionCreate {
    id: u64,
}

#[get("/collection/{name}/{id}")]
async fn collection_get(
    path: web::Path<(String, u64)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (name, id) = path.into_inner();
    let body = data.repository.get(name, id);
    return HttpResponse::Ok().body(body);
}

#[put("/collection/{name}")]
async fn collection_create(
    json: web::Json<Value>,
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = data.repository.create(path.into_inner(), json.into_inner());
    return HttpResponse::Ok().json(CollectionCreate { id });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let platform = new_default_platform(0, false).make_shared();

    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut Isolate::new(Default::default());
    let mut handle_scope = HandleScope::new(isolate);

    let query_engine = &mut QueryEngine::new(&mut handle_scope);

    query_engine.call("name");

    let app_state = web::Data::new(AppState {
        repository: Repository::new(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(collection_get)
            .service(collection_create)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
