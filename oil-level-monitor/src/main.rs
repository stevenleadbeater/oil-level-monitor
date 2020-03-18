mod logger;
mod model;
mod websocket;
mod mapper;
mod repository;

use actix_web_actors::ws;
use actix_web::{post, get, web, HttpResponse, HttpServer, App, HttpRequest, Error};
use actix_cors::Cors;
use log::{info, debug};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::broadcast;

use crate::logger::setup_logger;
use crate::websocket::WebSocket;
use crate::repository::distance_repository;
use crate::repository::distance_history_repository;
use crate::mapper::{distance_mapper, distance_history_mapper};
use crate::model::distance::Distance;
use std::time::{SystemTime, Duration};
use crate::model::distance_history::DistanceHistory;

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

    match distance_repository::get_by_id(id.into_inner()).await {
        Ok(rows) =>
            match distance_mapper::map(rows) {
                Ok(distance) => HttpResponse::Ok().body(serde_json::to_string(&distance).unwrap()),
                Err(error) => HttpResponse::InternalServerError().body(error)
            },
        Err(error) => HttpResponse::InternalServerError().body(error)
    }
}

#[get("/{id}/history")]
async fn get_history_by_id(id: web::Path<i32>) -> HttpResponse {
    info!("GET /{}/history", id);

    match distance_history_repository::get_by_distance_id(id.into_inner()).await {
        Ok(rows) =>
            match distance_history_mapper::map_many(rows) {
                Ok(distances) => HttpResponse::Ok().body(serde_json::to_string(&distances).unwrap()),
                Err(error) => HttpResponse::InternalServerError().body(error)
            },
        Err(error) => HttpResponse::InternalServerError().body(error)
    }
}

#[post("/")]
async fn post(
    item: web::Json<Distance>,
    tx: web::Data<Mutex<tokio::sync::broadcast::Sender<Distance>>>,
) -> HttpResponse {
    let distance = item.into_inner();
    info!("POST / {:#?}", distance);

    if let Err(error) = distance_repository::upsert(distance).await {
        return HttpResponse::InternalServerError().body(error);
    }

    let latest = distance_history_repository::get_latest_distance_history(distance.id).await;
    if latest.is_err() {
        return HttpResponse::InternalServerError().body(latest.err().unwrap());
    }
    let latest = latest.unwrap();
    if latest.len() > 0 {
        let latest = distance_history_mapper::map_one(latest);
        if latest.is_err() {
            return HttpResponse::InternalServerError().body(latest.err().unwrap());
        }
        let time_since_last_reading = SystemTime::now().duration_since(latest.unwrap().time_of_reading);
        if time_since_last_reading.is_err() {
            return HttpResponse::InternalServerError().body("Failed to calculate time since last reading");
        }
        let time_since_last_reading = time_since_last_reading.unwrap();
        if time_since_last_reading > Duration::from_secs(3600) {
            if let Err(error) = record_distance_history(distance).await {
                return HttpResponse::InternalServerError().body(error);
            }
        }
    } else if latest.len() == 0 {
        if let Err(error) = record_distance_history(distance).await {
            return HttpResponse::InternalServerError().body(error);
        }
    }
    tx.lock().unwrap().send(distance).unwrap();
    HttpResponse::Ok().finish()
}

async fn record_distance_history(distance: Distance) -> Result<(), String> {
    distance_history_repository::insert(DistanceHistory {
        id: None,
        distance_id: distance.id,
        distance: distance.distance,
        time_of_reading: SystemTime::now()
    }).await
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
        .service(get_history_by_id)
        .service(web::resource("/ws/").route(web::get().to(ws_index))))
        .bind("0.0.0.0:8120")?
        .run()
        .await
}
