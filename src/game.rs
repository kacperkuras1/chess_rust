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



impl GameStatus{
    pub fn as_str(&self) -> &'static str {
        match self {
            GameStatus::Waiting => "waiting",
            GameStatus::InProgress => "in_progress",
            GameStatus::WhiteWin => "white_win",
            GameStatus::BlackWin => "black_win",
            GameStatus::Draw => "draw",
            GameStatus::Ended => "ended",
        }
    }
}



pub struct Game {
    pub game_id: i32,
    pub white: Player,
    pub black: Player,
    pub board: Board,
    pub move_number: i32,
    pub status: GameStatus,
}

impl Game {

    pub fn new(game_id: i32, white: Player, black: Player) -> Self {
        Game {
            game_id,
            white,
            black,
            board: Board::default(),
            move_number: 1,
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

pub fn calculate_elo_changes(white_elo: i32, black_elo: i32, result: &GameStatus) -> (i32, i32) {
    let k = 32.0; //nazazie stały ale to do zmmiany w zależności od rankingu

    let expected_white = 1.0 / (1.0 + 10f64.powf(((black_elo - white_elo) as f64) / 400.0));
    let expected_black = 1.0 / (1.0 + 10f64.powf(((white_elo - black_elo) as f64) / 400.0));

    let (score_white, score_black) = match result {
        GameStatus::WhiteWin => (1.0, 0.0),
        GameStatus::BlackWin => (0.0, 1.0),
        GameStatus::Draw => (0.5, 0.5),
        _ => (0.0, 0.0),
    };

    let white_change = (k * (score_white - expected_white)).round() as i32;
    let black_change = (k * (score_black - expected_black)).round() as i32;

    (white_change, black_change)
}


#[derive(Default)]
pub struct GameState {
    pub queue: Vec<Player>,
    pub games: Vec<Game>,
}

pub type SharedGameState = std::sync::Arc<std::sync::Mutex<GameState>>;