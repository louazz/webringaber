use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    dir: String,
}

pub async fn navigate(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();

    if let Ok(params) = web::Query::<Params>::from_query(req.query_string()) {
        web::Redirect::to("https://longdogechallenge.com")
    } else {
        web::Redirect::to("/invalid_dir")
    }
}

pub async fn invalid_dir_error(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("<h1>An invalid direction was given to the server</h1><h2>Contact the author of the website you just came from and let them know :)</h2>")
}
