use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::site_indexer::AppState;

#[derive(Debug, Deserialize)]
pub struct Params {
    dir: String,
}

pub async fn navigate(req: HttpRequest, path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    println!("{}",user_id);
    // check if user id is available on the server
    println!("{}", data.sites[0].0);

    let mut found = false;
    let mut item_url="";
    for item in data.sites.iter(){
       if item.0 == user_id {
        found = true;
        item_url = &item.1;
        break;
       }
    }
    if found == true {
        web::Redirect::to(item_url.to_string())

    }else{
        web::Redirect::to("/invalid_dir")

    }
}

pub async fn invalid_dir_error(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("<h1>An invalid direction was given to the server</h1><h2>Contact the author of the website you just came from and let them know :)</h2>")
}
