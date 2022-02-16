
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub mod chat;
pub mod server;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ClientMessage {
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
    },
    // #[serde(rename_all = "camelCase")]
    // CreateMoonBang {
    //     id: String,
    //     name: String,
    //     width: i32,
    //     height: i32,
    //     private: bool,
    //     #[serde(flatten)]
    //     extra: HashMap<String, Value>,
    // }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    PlayerRegistered {
        id: String,
        name: String,
        x: f32,
        y: f32,
        rotation: f32,
    },
    PlayerRespawned {
        id: String,
        x: f32,
        y: f32,
        rotation: f32,
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
    #[serde(rename_all = "camelCase")]
    Asteroid {
         id: String,
         radius: f32,
         points: Vec<f32>,
         velocity_x: f32,
         velocity_y: f32,
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

pub fn create_asteroid() -> ServerMessage {
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
    let velocity_x = rng.gen_range(-1.0, 1.0);
    let velocity_y = rng.gen_range(-1.0, 1.0);
    ServerMessage::Asteroid{ id: String::from("uuid"), radius, points, velocity_x, velocity_y}
}

#[derive(Serialize, Deserialize)]
pub struct MoonBang {
    name: String,
    width: i32,
    height: i32,
    private: bool,
}
