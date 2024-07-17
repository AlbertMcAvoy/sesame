
use std::time::Instant;
use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::ws::{server::Server, session::WsSession};

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}