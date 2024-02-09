use actix_web::{get, middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    dir: String,
}

#[get("/{user_id}")]
pub async fn navigate(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
    HttpResponse::Ok().body(format!("{:?}", params))
}
