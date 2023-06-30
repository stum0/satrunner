use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ServerMsg {
    GameState(GameState),
    ClientMsg(ClientMsg),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientMsg {
    pub input: InputVec2,
    pub index: usize,
    pub id: String,
}

impl ClientMsg {
    pub fn new(input: InputVec2, index: usize, id: String) -> Self {
        Self { input, index, id }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub local_pos: f32,
    pub other_pos: Vec<f32>,
    pub dots: Vec<Vec3>,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputVec2 {
    pub x: f32,
    pub y: f32,
}

impl InputVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
