use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn get_ip(req: HttpRequest) -> impl Responder {
    let ip = req
        .peer_addr()
        .map(|addr| format!("{}\n", addr.ip()))
        .unwrap_or_else(|| "Unknown".to_string());
    ip
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(get_ip)))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
