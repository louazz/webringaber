mod api;
mod site_indexer;

use actix_web::{self, web, App, HttpServer};
use api::endpoints::*;
use site_indexer::AppState;
use site_indexer::read_file;
use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting ..");
    let path = Path::new("/home/louai/aber_webring/list.txt");
    let data =read_file(path);
    HttpServer::new(move || {
        App::new()
            .route("/user/{user_id}", web::get().to(navigate))
            .route("/invalid_dir", web::get().to(invalid_dir_error))
            .app_data(web::Data::new(AppState {
                sites: data.clone(),
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
