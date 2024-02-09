use actix_web::{
    cookie::time::Result, get, middleware::Logger, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    dir: String,
}

#[get("/invalid_dir")]
pub async fn invalid_dir_error(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("<h1>An error occurred! :<</h1>")
}

#[get("/{user_id}")]
pub async fn navigate(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();

    if let Ok(params) = web::Query::<Params>::from_query(req.query_string()) {
        web::Redirect::to("https://longdogechallenge.com")
    } else {
        web::Redirect::to("/invalid_dir") // TODO THIS DONT WORK
    }

    // if error
    // HttpResponse:Ok().body("an error occured :<, message the websites owner")
}


