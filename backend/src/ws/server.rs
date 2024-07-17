//! `Server` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `Server`.

use std::collections::{HashMap, HashSet};

use rand::{rngs::ThreadRng, Rng};
use actix::prelude::*;

use crate::models::database::AppState;

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
    pub toilet_id: usize,
    pub room: String,
    pub appState: AppState
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveMessage {
    pub session_id: usize,
    pub toilet_id: usize,
    pub room: String,
    pub appState: AppState
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
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // auto join session to main room
        self.rooms.entry("main".to_owned()).or_default().insert(id);

        // send id back
        id
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

    fn handle(&mut self, scanMessage: ScanMessage, _: &mut Context<Self>) {
        
    }
}

/// Handler for LeaveMessage.
impl Handler<LeaveMessage> for Server {
    type Result = ();

    fn handle(&mut self, leaveMessage: LeaveMessage, _: &mut Context<Self>) {
    }
}