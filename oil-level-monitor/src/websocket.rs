use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::Mutex;
use actix::prelude::*;
use actix_web_actors::ws;
use log::info;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct WebSocket<T: 'static> {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    pub payloads: Mutex<HashMap<i32, T>>,
    rx: tokio::sync::broadcast::Receiver<T>,
    get_subscribed_item: Option<Box<dyn Fn(T) -> bool>>,
}

impl<T> Actor for WebSocket<T> where T: std::marker::Unpin + std::clone::Clone + serde::ser::Serialize {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.start(ctx);
    }
}

/// Handler for `ws::Message`
impl<T> StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket<T> where T: std::marker::Unpin + std::clone::Clone + serde::ser::Serialize {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(_)) => self.hb = Instant::now(),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl<T> WebSocket<T> where T: std::marker::Unpin + std::clone::Clone + serde::ser::Serialize {
    pub fn new(payloads: Mutex<HashMap<i32, T>>, rx: tokio::sync::broadcast::Receiver<T>, get_subscribed_item: Option<Box<dyn Fn(T) -> bool>>) -> Self {
        Self { hb: Instant::now(), payloads, rx, get_subscribed_item }
    }

    fn send(&mut self, ctx: &mut <Self as Actor>::Context) {
        loop {
            let payload = match self.rx.try_recv() {
                Ok(payload) => {
                    info!("Receiving payload on channel");
                    if self.get_subscribed_item.is_some() && self.get_subscribed_item.as_ref().unwrap()(payload.clone()) {
                        payload
                    } else {
                        payload
                    }
                }
                Err(_) => {
                    return;
                }
            };
            ctx.text(serde_json::to_string(&payload).expect("cannot serialize"));
        }
    }

    fn evacuate(&mut self) {
        loop {
            match self.rx.try_recv() {
                Ok(_) => {
                    info!("Receiving payload on channel");
                }
                Err(_) => {
                    return;
                }
            };
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn start(&mut self, ctx: &mut <Self as Actor>::Context) {
        for payload in self.payloads.lock().unwrap().values() {
            ctx.text(serde_json::to_string(&payload).expect("cannot serialize"));
        }
        self.evacuate();
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            act.send(ctx);
        });
    }
}
