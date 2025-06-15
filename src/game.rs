use actix::Addr;
use chess::{Board, ChessMove, MoveGen, Square, Color, GameResult};
use crate::ws::ChessSession;

#[derive(Clone)]
pub struct Player {
    pub user_id: i32,
    pub username: String,
    pub elo: i32,
    pub addr: Addr<ChessSession>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    pub board: Board,
    pub status: GameStatus,
}

pub enum MoveOutcome {
    Ok,
    Checkmate(Color),
    Stalemate,
}

impl Game {

    pub fn new(white: Player, black: Player) -> Self {
        Game {
            white,
            black,
            board: Board::default(),
            status: GameStatus::InProgress,
        }
    }

    pub fn validate_and_play_move(&mut self, user_id: i32, from: &str, to: &str) -> Result<(), (&'static str, String)> {
        let player_color = if self.white.user_id == user_id {
            Color::White
        } else if self.black.user_id == user_id {
            Color::Black
        } else {
            return Err(("Nie jesteś uczestnikiem tej gry", self.board.to_string()));
        };

        if self.board.side_to_move() != player_color {
            return Err(("Nie twoja kolej", self.board.to_string()));
        }

        let from_sq = Square::from_string(String::from(from)).unwrap();
        let to_sq = Square::from_string(String::from(to)).unwrap();

        let mv = ChessMove::new(from_sq, to_sq, None);
        let mut legal_moves = MoveGen::new_legal(&self.board);

        if legal_moves.any(|m| m == mv) {
            self.board = self.board.make_move_new(mv);

            // Sprawdź, czy gra się skończyła
            let next_moves = MoveGen::new_legal(&self.board);
            if next_moves.len() == 0 {
                if self.board.checkers().popcnt() > 0 {
                    self.status = match self.board.side_to_move() {
                        Color::White => GameStatus::BlackWin,
                        Color::Black => GameStatus::WhiteWin,
                    };
                } else {
                    self.status = GameStatus::Draw;
                }
            }

            Ok(())
        } else {
            Err(("Nielegalny ruch", self.board.to_string()))
        }
    }
}


#[derive(Default)]
pub struct GameState {
    pub queue: Vec<Player>,
    pub games: Vec<Game>,
}

pub type SharedGameState = std::sync::Arc<std::sync::Mutex<GameState>>;