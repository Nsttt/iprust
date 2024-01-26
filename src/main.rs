use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use dotenv::dotenv;
use log::{info, warn};
use std::{env, io};

async fn get_ip(req: HttpRequest) -> impl Responder {
    if let Some(addr) = req.peer_addr() {
        info!("Received request from {}", addr);
        format!("{}\n", addr.ip())
    } else {
        warn!("Could not get client IP");
        "Unknown\n".to_string()
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    info!("Starting server at {}", &bind_address);

    HttpServer::new(|| App::new().route("/", web::get().to(get_ip)))
        .bind(&bind_address)?
        .run()
        .await
}
