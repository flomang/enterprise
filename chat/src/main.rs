use std::sync::{
    atomic::{AtomicUsize},
    Arc,
};
use std::time::{Duration, Instant};
use std::env;
use std::collections::HashMap;


use actix::*;
use actix_cors::Cors;
use actix_web::{http, web, middleware::Logger, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use rand::Rng;


mod server;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

struct WsChatSession {
    /// unique session id
    id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    /// joined room
    room: String,
    /// peer name
    name: Option<String>,
    /// Chat server
    addr: Addr<server::ChatServer>,
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

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(tag = "type")]
enum ClientMessage {
    RegisterPlayer {
        id: String,
        name: String,
        x: f32,
        y: f32,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    RespawnPlayer {
        id: String,
        x: f32,
        y: f32,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    PlayerDied {
        id: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename_all = "camelCase")]
    PlayerKeyboardArrowUp {
        id: String,
        key_down: bool,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename_all = "camelCase")]
    PlayerKeyboardArrowLeft {
        id: String,
        key_down: bool,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename_all = "camelCase")]
    PlayerKeyboardArrowRight {
        id: String,
        key_down: bool,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
enum ServerMessage {
    PlayerRegistered {
        id: String,
        name: String,
        x: f32,
        y: f32,
    },
    PlayerRespawned {
        id: String,
        x: f32,
        y: f32,
    },
    PlayerDied {
        id: String,
    },
    #[serde(rename_all = "camelCase")]
    PlayerMoveForward {
        id: String,
        is_moving: bool,
    },
    #[serde(rename_all = "camelCase")]
    PlayerRotateLeft {
        id: String,
        is_rotating: bool,
    },
    #[serde(rename_all = "camelCase")]
    PlayerRotateRight {
        id: String,
        is_rotating: bool,
    },
    Asteroid {
         id: String,
         radius: f32,
         points: Vec<f32>,
    },
}

fn clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min {
        min
    } else if input > max {
        max 
    } else {
        input
    } 
}
  
fn map(current: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    let mapped: f32 = ((current - in_min) * (out_max - out_min)) / (in_max - in_min) + out_min;
    return clamp(mapped, out_min, out_max);
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

        println!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();

                // TODO move this into a struct
                let mut rng = rand::thread_rng();
                let total = rng.gen_range(6, 12);
                let radius: f32 = rng.gen_range(3.0, 21.0);
                let mut points = Vec::new();

                for i in 0..total {
                    let angle = map(i as f32, 0.0, total as f32, 0.0, (std::f64::consts::PI * 2.0) as f32 );
                    let offset = rng.gen_range(-radius * 0.5, radius * 0.5);
                    let r = radius + offset;
                    let x = r * angle.cos();
                    let y = r * angle.sin();

                    points.push(x);
                    points.push(y);
                }
                let asteroid = ServerMessage::Asteroid{ id: String::from("uuid"), radius: radius, points: points};
                let json = serde_json::to_string(&asteroid).unwrap();
                self.addr.do_send(server::ClientMessage {
                    id: self.id,
                    msg: json,
                    room: self.room.clone(),
                });
            }
            ws::Message::Text(text) => {
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
                                                    let msg = ServerMessage::PlayerRegistered {id, name, x, y};
                                                    let json = serde_json::to_string(&msg).unwrap();
                                                    // send message to chat server
                                                    self.addr.do_send(server::ClientMessage {
                                                         id: self.id,
                                                         msg: json,
                                                         room: self.room.clone(),
                                                    });
                                                },

                                                ClientMessage::RespawnPlayer{ id, x, y, extra: _ } => {
                                                    //println!("new player: {} ({})", id, name)
                                                    let msg = ServerMessage::PlayerRespawned {id, x, y};
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // log info 
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // App state
    // We are keeping a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // Start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();

    // Create Http server with websocket support
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&env::var("CLIENT_HOST").unwrap())
            .allow_any_method()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(app_state.clone())
            .data(server.clone())
            // websocket
            .service(web::resource("/ws/").to(chat_route))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}