use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct UrlMapping {
    original: String,
    short: String,
}

struct AppState {
    url_mappings: Mutex<Vec<UrlMapping>>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("URL Shortener Service")
}

async fn shorten_url(data: web::Data<AppState>, url: web::Json<UrlMapping>) -> impl Responder {
    let mut url_mappings = data.url_mappings.lock().unwrap();
    let short_url = Uuid::new_v4().to_string();
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        url_mappings: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/shorten", web::post().to(shorten_url))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
