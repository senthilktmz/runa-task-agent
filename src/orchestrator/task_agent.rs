use std::any::Any;
use actix_web::{web, HttpResponse};
use runautils::actix_server_util::{ServerStateStore};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

async fn post_req(body: web::Json<String>, path: &'static str) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "received": *body, "path": path }))
}

pub fn post_handler(
    body: web::Json<String>,
    path: &'static str,
    server_context: Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: Arc<Mutex<ServerStateStore>>,
) -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(post_req(body, path))
}
