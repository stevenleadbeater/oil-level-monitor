mod logger;
mod distance;

use distance::Distance;
use actix_web::{post, get, web, HttpResponse, HttpServer, App};
use actix_cors::Cors;
use log::info;
use crate::logger::setup_logger;

#[get("/")]
async fn get() -> HttpResponse {
    info!("GET /");
    HttpResponse::Ok().finish()
}

#[post("/")]
async fn post(item: web::Json<Distance>) -> HttpResponse {
    let distance = item.into_inner();
    info!("POST / {:#?}", distance);
    HttpResponse::Ok().finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    setup_logger(true, None);
    info!("Starting distance listener");

    HttpServer::new(move || App::new()
        .wrap(Cors::new().send_wildcard().finish())
        .service(get)
        .service(post))
        .bind("0.0.0.0:8120")?
        .run()
        .await
}
