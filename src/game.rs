use actix::Addr;
use crate::ws::ChessSession;

#[derive(Clone)]
pub struct Player {
    pub user_id: i32,
    pub username: String,
    pub elo: i32,
    pub addr: Addr<ChessSession>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Waiting,
    InProgress,
    WhiteWin,
    BlackWin,
    Draw,
    Ended,
}

pub struct Game {
    pub white: Player,
    pub black: Player,
    pub status: GameStatus,
}

#[derive(Default)]
pub struct GameState {
    pub queue: Vec<Player>,
    pub games: Vec<Game>,
}

pub type SharedGameState = std::sync::Arc<std::sync::Mutex<GameState>>;