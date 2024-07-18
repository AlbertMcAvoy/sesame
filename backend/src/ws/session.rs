use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws::{self, CloseCode, CloseReason};

use crate::models::database::AppState;

use super::server;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub enum ActionSession {
    ClientScan,
    ClientLeave,
    Unkown,
}

impl ActionSession {
    fn matching(pattern: &str) -> Self {
        match pattern {
            "client-scan" => ActionSession::ClientScan,
            "client-leave" => ActionSession::ClientLeave,
            _ => ActionSession::Unkown,
        }
    }
}

#[derive(Debug)]
pub struct WsSession {
    /// unique session id
    pub id: usize,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,

    /// server
    pub addr: Addr<server::Server>,

    /// AppState
    pub state: AppState,
}

impl WsSession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify server
                act.addr.do_send(server::Disconnect { session_id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with Server
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify server
        self.addr.do_send(server::Disconnect {
            session_id: self.id,
        });
        Running::Stop
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                let v: Vec<&str> = m.splitn(3, ' ').collect();
                let action = ActionSession::matching(v[0]);

                match action {
                    ActionSession::ClientScan => match v.len() {
                        3 => match v[1].parse::<i32>() {
                            Ok(toilet_id) => self.addr.do_send(server::ScanMessage {
                                session_id: self.id,
                                toilet_id,
                                scan_mode: v[2].to_owned(),
                                app_state: self.state.to_owned(),
                            }),
                            Err(e) => ctx.text(format!("!!! Invalid toilet id : {}", e)),
                        },
                        _ => ctx.text(format!("!!! Invalid number of arguments")),
                    },
                    ActionSession::ClientLeave => match v.len() {
                        2 => match v[1].parse::<i32>() {
                            Ok(toilet_id) => {
                                match self.addr.try_send(server::LeaveMessage {
                                    session_id: self.id,
                                    toilet_id,
                                    app_state: self.state.to_owned(),
                                }) {
                                    Ok(_) => {
                                        ctx.text(format!("ENDED"));
                                        ctx.close(Some(CloseReason {
                                            code: CloseCode::Normal,
                                            description: Some("END".to_string()),
                                        }));
                                        ctx.stop();
                                    }
                                    Err(_) => {}
                                }
                            }
                            Err(_) => ctx.text(format!("!!! Invalid toilet id")),
                        },
                        _ => ctx.text(format!("!!! Invalid number of arguments")),
                    },
                    ActionSession::Unkown => ctx.text(format!("!!! Unknown command: {m:?}")),
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

/// Handle messages from server, we simply send it to peer websocket
impl Handler<server::Message> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
