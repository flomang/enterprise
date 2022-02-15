use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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

pub fn clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min {
        min
    } else if input > max {
        max 
    } else {
        input
    } 
}
  
pub fn map(current: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    let mapped: f32 = ((current - in_min) * (out_max - out_min)) / (in_max - in_min) + out_min;
    return clamp(mapped, out_min, out_max);
}
