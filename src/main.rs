mod call_function_with_context_transformer;
mod query_engine;
mod repository;
mod stages;
mod utils;

use std::{collections::HashMap, sync::mpsc};

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use query_engine::{QueryEngineCall, QUERY_ENGINE};
use repository::REPOSITORY;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
struct CollectionCreate {
    id: String,
}

#[get("/collection/{name}/{id}")]
async fn collection_get(
    path: web::Path<(String, String)>
) -> impl Responder {
    let (name, id) = path.into_inner();
    let body = REPOSITORY.get(name, id);
    return HttpResponse::Ok().content_type("application/json").body(body);
}

#[put("/collection/{name}")]
async fn collection_create(
    json: web::Json<Value>,
    path: web::Path<String>
) -> impl Responder {
    let id = REPOSITORY.create(path.into_inner(), json.into_inner());
    return HttpResponse::Ok().json(CollectionCreate { id });
}

#[post("/collection/{name}/{id}")]
async fn collection_update(
    json: web::Json<Value>,
    path: web::Path<(String, String)>
) -> impl Responder {
    let (name, id) = path.into_inner();
    REPOSITORY.update(name, id, json.into_inner());
    return HttpResponse::Ok();
}

#[delete("/collection/{name}/{id}")]
async fn collection_delete(
    path: web::Path<(String, String)>
) -> impl Responder {
    let (name, id) = path.into_inner();
    REPOSITORY.delete(name, id);
    return HttpResponse::Ok();
}

#[get("/query/{name}")]
async fn query(
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>
) -> impl Responder {
    let (result, receiver) = mpsc::channel::<String>();

    QUERY_ENGINE.call.send(QueryEngineCall {
        name: path.into_inner(),
        args: query.0,
        result
    }).unwrap();
    return HttpResponse::Ok()
    .content_type("application/json")
    .body(receiver.recv().unwrap());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(collection_get)
            .service(collection_create)
            .service(collection_update)
            .service(collection_delete)       
            .service(query)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
