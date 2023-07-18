use std::collections::VecDeque;

use bevy::{prelude::*, utils::Instant};
use futures::channel::mpsc::{Receiver, Sender};

use crate::network::messages::ClientMessage;

//dots
#[derive(Resource)]
pub struct Dots {
    pub pos: Vec<Vec3>,
    pub rng_seed: Option<u64>,
}

impl Dots {
    pub fn new() -> Self {
        Self {
            pos: Vec::new(),
            rng_seed: None,
        }
    }
}

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);

//server
#[derive(Resource)]
pub struct NetworkStuff {
    pub write: Option<Sender<ClientMessage>>,
    pub read: Option<Receiver<Vec<u8>>>,
}

impl NetworkStuff {
    pub fn new() -> Self {
        Self {
            write: None,
            read: None,
        }
    }
}

#[derive(Resource)]
pub struct ClientTick {
    pub tick: u64,
    pub time: Instant,
    pub pause: i64,
}

impl ClientTick {
    pub fn new() -> Self {
        Self {
            tick: 0,
            time: Instant::now(),
            pause: 0,
        }
    }
}

#[derive(Resource)]
pub struct PlayerName {
    pub name: String,
    pub submitted: bool,
}

impl PlayerName {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            submitted: false,
        }
    }
}
