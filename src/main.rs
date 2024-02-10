mod api;
mod site_indexer;

use actix_web::{self, web, App, HttpServer};
use api::endpoints::*;
use site_indexer::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{user_id}", web::get().to(invalid_dir_error))
            .route("/invalid_dir", web::get().to(navigate))
            .app_data(web::Data::new(AppState {
                sites: vec![("user_id".to_string(), "url".to_string())],
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
