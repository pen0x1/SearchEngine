use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/greet", web::get().to(greet)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    println!("Starting server at: {}", &server_addr);
    HttpServer::new(|| {
        App::new()
            .configure(config_routes)
    })
    .bind(&server_addr)?
    .run()
    .await
}