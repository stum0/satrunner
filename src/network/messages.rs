use speedy::{Readable, Writable};
use uuid::Uuid;

// Network messages
#[derive(Readable, Writable, Debug, Clone)]
pub enum NetworkMessage {
    GameUpdate(Vec<NewPos>),
    GameState(Vec<PlayerState>),
    NewGame(NewGame),
    Ping,
    DamagePlayer(Damage),
    ScoreUpdate(Score),
    SyncClient(SyncMessage),
}

#[derive(Readable, Writable, Debug, Clone)]
pub enum ClientMessage {
    PlayerInput(PlayerInput),
    PlayerName(String),
}

#[derive(Readable, Writable, Debug, Clone, Default)]
pub struct NewPos {
    pub input: [f32; 2],
    pub tick: u64,
    pub id: Uuid,
    pub pos: [f32; 2],
}

#[derive(Readable, Writable, Debug, Clone, Default)]
pub struct SyncMessage {
    pub tick_adjustment: i64,
    pub server_tick: u64,
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct PlayerInput {
    pub target: [f32; 2],
    pub id: Uuid,
    pub tick: u64,
    pub in_game: bool,
}

impl PlayerInput {
    pub fn new(target: [f32; 2], id: Uuid, tick: u64, in_game: bool) -> Self {
        Self {
            target,
            id,
            tick,
            in_game,
        }
    }
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct NewGame {
    pub id: Uuid,
    pub server_tick: u64,
    pub rng_seed: u64,
    pub high_scores: Vec<(String, u64)>,
    pub objects: ObjectMsg,
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct ObjectMsg {
    pub rain_pos: Vec<(u64, [f32; 2])>,
    pub bolt_pos: Vec<(u64, [f32; 2])>,
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct Damage {
    pub id: Uuid,
    pub tick: Option<u64>,
    pub secs_alive: u64,
    pub high_scores: Option<Vec<(String, u64)>>,
    pub pos: [f32; 2],
    pub score: usize,
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct Score {
    pub id: Uuid,
    pub score: usize,
    pub tick: u64,
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct PlayerState {
    pub pos: [f32; 2],
    pub target: [f32; 2],
    pub score: usize,
    pub name: Option<String>,
    pub id: Uuid,
    pub time_alive: u64,
    pub alive: bool,
}
