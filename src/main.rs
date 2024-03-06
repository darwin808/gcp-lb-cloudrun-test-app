use actix_web::{get, App, HttpServer, Responder};
use std::env;

#[get("/")]
async fn greet() -> impl Responder {
    format!("Hello World Zesty 👋!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read the PORT environment variable, default to 8080 if not set
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(|| App::new().service(greet))
        .bind(("0.0.0.0", port.parse().expect("Invalid PORT")))
        .expect("Failed to bind to address")
        .run()
        .await
}
