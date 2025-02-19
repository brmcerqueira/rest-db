mod query_engine;
mod repository;
mod stages;
mod typescript;
mod utils;
mod try_catch_verify;
mod local_array_extension;

use crate::typescript::ts_transpiler::ts_transpiler;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use query_engine::{QueryEngineCall, QUERY_ENGINE};
use repository::REPOSITORY;
use serde_json::Value;
use std::{collections::HashMap, sync::mpsc};
use v8::new_default_platform;
use v8::V8::{initialize, initialize_platform};

#[derive(Debug, MultipartForm)]
struct UploadScript {
    #[multipart(limit = "100MB")]
    script: TempFile,
}

#[get("/collection/{name}/{id}")]
async fn collection_get(path: web::Path<(String, String)>) -> impl Responder {
    let (name, id) = path.into_inner();
    let body = REPOSITORY.get(name, id);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[put("/collection/{name}")]
async fn collection_create(json: web::Json<Value>, path: web::Path<String>) -> impl Responder {
    let id = REPOSITORY.create(path.into_inner(), json.into_inner());
    HttpResponse::Ok().json(id)
}

#[post("/collection/{name}/{id}")]
async fn collection_update(
    json: web::Json<Value>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (name, id) = path.into_inner();
    REPOSITORY.update(name, id, json.into_inner());
    HttpResponse::Ok()
}

#[delete("/collection/{name}/{id}")]
async fn collection_delete(path: web::Path<(String, String)>) -> impl Responder {
    let (name, id) = path.into_inner();
    REPOSITORY.delete(name, id);
    HttpResponse::Ok()
}

#[get("/query/{name}")]
async fn query(
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let (result, receiver) = mpsc::channel::<String>();

    QUERY_ENGINE
        .lock()
        .unwrap()
        .call
        .send(QueryEngineCall {
            name: path.into_inner(),
            args: query.0,
            result,
        })
        .unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(receiver.recv().unwrap())
}

#[put("/script/{main}")]
async fn upload_script(
    path: web::Path<String>,
    MultipartForm(form): MultipartForm<UploadScript>,
) -> impl Responder {
    ts_transpiler(form.script.file, path.into_inner());
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let platform = new_default_platform(0, false).make_shared();

    initialize_platform(platform);

    initialize();

    HttpServer::new(move || {
        App::new()
            .service(upload_script)
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
