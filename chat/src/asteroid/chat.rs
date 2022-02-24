use std::time::{Duration, Instant};
use actix::*;
use actix_web_actors::ws;

use super::server;
use super::*;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(3);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


pub struct WsChatSession {
    /// unique session id
    pub id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,
    /// joined room
    pub room: String,
    /// peer name
    pub name: Option<String>,
    /// Chat server
    pub addr: Addr<server::ChatServer>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
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
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
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

                let asteroid = create_asteroid(); 
                let json = serde_json::to_string(&asteroid).unwrap();
                self.addr.do_send(server::ClientMessage{
                    id: self.id,
                    msg: json,
                    room: self.room.clone(),
                });
            }
            ws::Message::Text(text) => {
                println!("WEBSOCKET MESSAGE: {:?}", text);
                let message = text.trim();
                // we check for /sss type of messages
                if message.starts_with('/') {
                    let args: Vec<&str> = message.splitn(2, ' ').collect();
                    match args[0] {
                        "/messages" => {
                            let values: Result<Vec<serde_json::Value>, serde_json::Error> = 
                                serde_json::from_str(args[1]);

                            if let Ok(values) = values {
                                for val in values {
                                    // if there is a type attribute 
                                    if let Some(_) = val.get("type") {
                                        let msg: Result<ClientMessage, serde_json::Error> = serde_json::from_value(val.clone());

                                        if let Ok(msg) = msg {
                                            match msg {
                                                ClientMessage::RegisterPlayer{ id, name, x, y, extra: _ } => {
                                                    let mut rng = rand::thread_rng();
                                                    let rotation = rng.gen_range(0.0, std::f64::consts::PI * 2.0) as f32;
                                                    let msg = ServerMessage::PlayerRegistered {id, name, x, y, rotation};
                                                    let json = serde_json::to_string(&msg).unwrap();
                                                    // send message to chat server
                                                    self.addr.do_send(server::ClientMessage {
                                                         id: self.id,
                                                         msg: json,
                                                         room: self.room.clone(),
                                                    });
                                                },

                                                ClientMessage::RespawnPlayer{ id, x, y, extra: _ } => {
                                                    let mut rng = rand::thread_rng();
                                                    let rotation = rng.gen_range(0.0, std::f64::consts::PI * 2.0) as f32;
                                                    let msg = ServerMessage::PlayerRespawned {id, x, y, rotation};
                                                    let json = serde_json::to_string(&msg).unwrap();
                                                    self.addr.do_send(server::ClientMessage {
                                                        id: self.id,
                                                        msg: json,
                                                        room: self.room.clone(),
                                                    });
                                                },

                                                ClientMessage::PlayerDied{ id, extra: _ } => { 
                                                    let msg = ServerMessage::PlayerDied {id};
                                                    let json = serde_json::to_string(&msg).unwrap();
                                                    self.addr.do_send(server::ClientMessage {
                                                        id: self.id,
                                                        msg: json,
                                                        room: self.room.clone(),
                                                    });
                                                }

                                                ClientMessage::PlayerKeyboardArrowUp{ id, key_down, extra: _ } => { 
                                                    let msg = ServerMessage::PlayerMoveForward {id, is_moving: key_down};
                                                    let json = serde_json::to_string(&msg).unwrap();
                                                    self.addr.do_send(server::ClientMessage {
                                                        id: self.id,
                                                        msg: json,
                                                        room: self.room.clone(),
                                                    });
                                                }

                                                ClientMessage::PlayerKeyboardArrowLeft{ id, key_down, extra: _ } => { 
                                                    let msg = ServerMessage::PlayerRotateLeft {id, is_rotating: key_down};
                                                    let json = serde_json::to_string(&msg).unwrap();
                                                    self.addr.do_send(server::ClientMessage {
                                                        id: self.id,
                                                        msg: json,
                                                        room: self.room.clone(),
                                                    });
                                                }

                                                ClientMessage::PlayerKeyboardArrowRight{ id, key_down, extra: _ } => { 
                                                    let msg = ServerMessage::PlayerRotateRight {id, is_rotating: key_down};
                                                    let json = serde_json::to_string(&msg).unwrap();
                                                    self.addr.do_send(server::ClientMessage {
                                                        id: self.id,
                                                        msg: json,
                                                        room: self.room.clone(),
                                                    });
                                                }
                                            }
                                        } else {
                                            println!("Unknown type encountered: {}", val.to_string());
                                        }
                                    } else {
                                        println!("Type property not found: {}", val.to_string());
                                    }
                                }
                            } else {
                                println!("Error in {:?}", values);
                            }
                        }
                        "/list" => {
                            // Send ListRooms message to chat server and wait for
                            // response
                            println!("List rooms");
                            self.addr
                                .send(server::ListRooms)
                                .into_actor(self)
                                .then(|res, _, ctx| {
                                    match res {
                                        Ok(rooms) => {
                                            for room in rooms {
                                                ctx.text(room);
                                            }
                                        }
                                        _ => println!("Something is wrong"),
                                    }
                                    fut::ready(())
                                })
                                .wait(ctx)
                            // .wait(ctx) pauses all events in context,
                            // so actor wont receive any new messages until it get list
                            // of rooms back
                        }
                        "/join" => {
                            if args.len() == 2 {
                                self.room = args[1].to_owned();
                                self.addr.do_send(server::Join {
                                    id: self.id,
                                    name: self.room.clone(),
                                });

                                ctx.text("joined");
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }
                        "/name" => {
                            if args.len() == 2 {
                                self.name = Some(args[1].to_owned());
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        _ => ctx.text(format!("!!! unknown command: {:?}", message)),
                    }
                } else {
                    let msg = if let Some(ref name) = self.name {
                        format!("{}: {}", name, message)
                    } else {
                        message.to_owned()
                    };
                    // send message to chat server
                    self.addr.do_send(server::ClientMessage {
                        id: self.id,
                        msg,
                        room: self.room.clone(),
                    })
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

impl WsChatSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(server::Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}