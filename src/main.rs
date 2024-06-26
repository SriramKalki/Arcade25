use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
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
    let new_mapping = UrlMapping {
        original: url.original.clone(),
        short: short_url.clone(),
    };
    url_mappings.push(new_mapping);

    HttpResponse::Ok().json(UrlMapping {
        original: url.original.clone(),
        short: short_url,
    })
}

async fn redirect(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let short_url = req.match_info().get("short_url").unwrap();
    let url_mappings = data.url_mappings.lock().unwrap();

    if let Some(mapping) = url_mappings.iter().find(|m| m.short == short_url) {
        HttpResponse::Found()
            .header("Location", &mapping.original)
            .finish()
    } else {
        HttpResponse::NotFound().body("URL not found")
    }
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
            .route("/{short_url}", web::get().to(redirect))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
