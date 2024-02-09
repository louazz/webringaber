mod api;

use actix_web::{self, App, HttpServer};
use api::endpoints::navigate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(navigate))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
