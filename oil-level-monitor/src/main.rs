mod logger;
mod distance;
mod websocket;

use distance::Distance;
use actix_web_actors::ws;
use actix_web::{post, get, web, HttpResponse, HttpServer, App, HttpRequest, Error};
use actix_cors::Cors;
use log::{info,debug};
use postgres::{Connection, TlsMode};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::broadcast;

use crate::logger::setup_logger;
use crate::websocket::WebSocket;

/// do websocket handshake and start `WebSocket` actor
async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    distances: web::Data<Mutex<HashMap<i32, Distance>>>,
    tx: web::Data<Mutex<tokio::sync::broadcast::Sender<Distance>>>,
) -> Result<HttpResponse, Error> {
    info!("{:?}", r);
    let cloned_distances = distances.lock().expect("Cannot lock shared rates for cloning").clone();
    let cloned_rx = tx.lock().expect("").subscribe();
    let res = ws::start(WebSocket::new(Mutex::new(cloned_distances), cloned_rx, None), &r, stream);
    debug!("{:?}", res);
    res
}

#[get("/{id}")]
async fn get_by_id(id: web::Path<i32>) -> HttpResponse {
    info!("GET /{}", id);

    let client = Connection::connect("postgresql://oil_level_user:password@192.168.1.245:5431/oil_level", TlsMode::None)
        .expect("Cannot connect to DB");

    let rows = client.query(r##"
    SELECT
        id, distance
    FROM
        distance
    WHERE
        id = $1
                            "##, &[&id.into_inner()])
        .expect("Query failed");

    if rows.len() != 1 {
        return HttpResponse::InternalServerError().body("Expecting exactly one row for queries by id");
    }

    if rows.get(0).len() != 2 {
        return HttpResponse::InternalServerError().body("Expecting exactly two columns for distance rows");
    }

    let id: i32 = rows.get(0).get(0);
    let distance: i32 = rows.get(0).get(1);

    let distance = Distance {
        id,
        distance,
    };

    HttpResponse::Ok().body(serde_json::to_string(&distance).unwrap())
}

#[post("/")]
async fn post(
    item: web::Json<Distance>,
    tx: web::Data<Mutex<tokio::sync::broadcast::Sender<Distance>>>
) -> HttpResponse {
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

    tx.lock().unwrap().send(distance).unwrap();
    HttpResponse::Ok().finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    setup_logger(true, None);
    info!("Starting distance listener");

    let distances: HashMap<i32, Distance> = HashMap::new();
    let shared_distances = web::Data::new(Mutex::new(distances));
    let (tx, _rx) = broadcast::channel::<Distance>(16);
    let publisher = web::Data::new(Mutex::new(tx));

    HttpServer::new(move || App::new()
        .app_data(shared_distances.clone())
        .app_data(publisher.clone())
        .wrap(Cors::new().send_wildcard().finish())
        .service(get_by_id)
        .service(post)
        .service(web::resource("/ws/").route(web::get().to(ws_index))))
        .bind("0.0.0.0:8120")?
        .run()
        .await
}
