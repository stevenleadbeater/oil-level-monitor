mod logger;
mod distance;

use distance::Distance;
use actix_web::{post, get, web, HttpResponse, HttpServer, App};
use actix_cors::Cors;
use log::info;
use crate::logger::setup_logger;
use postgres::{Connection, TlsMode};

#[get("/")]
async fn get() -> HttpResponse {
    info!("GET /");
    HttpResponse::Ok().finish()
}

#[post("/")]
async fn post(item: web::Json<Distance>) -> HttpResponse {
    let distance = item.into_inner();
    info!("POST / {:#?}", distance);

    let client = Connection::connect("postgresql://oil_level_user:password@192.168.1.245:5431/oil_level", TlsMode::None)
        .expect("Cannot connect to DB");

    client.execute(r##"
    INSERT INTO distance (id, distance) VALUES ($1, $2)
    ON CONFLICT (id)
    DO
        UPDATE
        SET distance = $2
        WHERE distance.id = $1;
                            "##, &[&distance.id, &distance.distance])
        .expect("Query failed");

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
