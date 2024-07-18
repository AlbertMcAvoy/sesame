//! `Server` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `Server`.

use std::collections::{HashMap, HashSet};

use rand::{rngs::ThreadRng, Rng};
use actix::prelude::*;

use diesel::{prelude::*, result::Error};
use crate::{models::{database::AppState, water_closet::WaterCloset}, schema::water_closets::{dsl::water_closets, id, is_available, is_door_locked, is_door_opened}};

///  server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for server communications

/// New session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ScanMessage {
    pub session_id: usize,
    pub toilet_id: i32,
    pub app_state: AppState
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveMessage {
    pub session_id: usize,
    pub toilet_id: i32,
    pub app_state: AppState
}

/// `Server` manages rooms and responsible for coordinating session.
///
/// Implementation is very naïve.
#[derive(Debug)]
pub struct Server {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
}

impl Server {
    pub fn new() -> Server {
        // default room
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());

        Server {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
        }
    }
}

impl Server {
    fn send_message(&self, message: &str, session_id: usize) {
        if let Some(addr) = self.sessions.get(&session_id) {
            addr.do_send(Message(message.to_owned()));
        }
    }
}

/// Make actor from `Server`
impl Actor for Server {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // register session with random id
        let session_id = self.rng.gen::<usize>();
        self.sessions.insert(session_id, msg.addr);

        // auto join session to main room
        self.rooms.entry("main".to_owned()).or_default().insert(session_id);

        // send id back
        session_id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.session_id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.session_id) {
                    rooms.push(name.to_owned());
                }
            }
        }
    }
}

/// Handler for ScanMessage.
impl Handler<ScanMessage> for Server {
    type Result = ();

    fn handle(&mut self, scan_message: ScanMessage, _: &mut Context<Self>) {
        let water_closet_result: Result<WaterCloset, Error> = water_closets.filter(id.eq(scan_message.toilet_id)).first::<WaterCloset>(&mut scan_message.app_state.get_conn());
        match water_closet_result {
            Ok(water_closet) => {
                let res = if water_closet.is_available {
                    let _ = diesel::update(water_closets.find(water_closet.id))
                        .set((
                            id.eq(&water_closet.id),
                            is_available.eq(false),
                            is_door_opened.eq(false),
                            is_door_locked.eq(false)
                        ));
                    "AVAILABLE"
                } else { "UNAVAILABLE" };
                self.send_message(res, scan_message.session_id)
            },
            Err(_) => self.send_message("UNKNOWN", scan_message.session_id)
        };
    }
}

/// Handler for LeaveMessage.
impl Handler<LeaveMessage> for Server {
    type Result = ();

    fn handle(&mut self, leave_message: LeaveMessage, _: &mut Context<Self>) {
        let water_closet_result: Result<WaterCloset, Error> = water_closets.filter(id.eq(leave_message.toilet_id)).first::<WaterCloset>(&mut leave_message.app_state.get_conn());
        match water_closet_result {
            Ok(water_closet) => {
                if !water_closet.is_available {
                    let _ = diesel::update(water_closets.find(water_closet.id))
                        .set((
                            id.eq(&water_closet.id),
                            is_available.eq(true),
                            is_door_opened.eq(false),
                            is_door_locked.eq(false)
                        ));

                    let mut rooms: Vec<String> = Vec::new();
                    // remove address
                    if self.sessions.remove(&leave_message.session_id).is_some() {
                        // remove session from all rooms
                        for (name, sessions) in &mut self.rooms {
                            if sessions.remove(&leave_message.session_id) {
                                rooms.push(name.to_owned());
                            }
                        }
                    }
                };
            },
            Err(_) => self.send_message("UNKNOWN", leave_message.session_id)
        };
    }
}