use core::time;
use std::collections::HashMap;

use crate::{
    core::{attack_pregen::PregenAttacks, board_state::BoardState, piece::Color, zobrist::ZobristHasher},
    move_logic::{move_encode::Move, move_eval, pseudo_move_gen},
};

use super::user_input;

/// Entry stored in the transposition table
#[derive(Clone, Debug)]
pub struct TTEntry {
    pub score: i32,
    pub depth: u8,
    pub entry_type: move_eval::EntryType,
}

/// Represents the current state of a chess game, with history and repetition tracking
pub struct GameState {
    /// The current board state
    board_state: BoardState,
    /// Pre-generated attack patterns for all pieces
    pregen_attacks: PregenAttacks,
    /// Zobrist hasher for position hashing
    zobrist: ZobristHasher,
    /// Transposition table for caching evaluated positions
    transposition_table: HashMap<u64, TTEntry>,
    /// Move history for undoing moves (old board states)
    history: Vec<BoardState>,
    /// Zobrist history for repetition detection
    zobrist_history: Vec<u64>,
    ///clock time (for time management)
    clock_time: std::time::Duration,
    /// Number of moves remaining in the game (for time management)
    moves_to_go: u32,
}

impl GameState {
    /// Creates a new game state from an optional FEN string and time limit
    pub fn new(fen_str: Option<&str>, time_limit: std::time::Duration) -> Self {
        let zobrist = ZobristHasher::new();
        let moves_to_go = match time_limit.as_secs() {
            0..=59 => 20,
            60..=179 => 30,
            180..=599 => 40,
            _ => 50,
        };

        // Initialize board state and compute initial hash
        let board = BoardState::new(fen_str, &zobrist).unwrap();
        let initial_hash = board.get_zobrist_hash();

        Self {
            board_state: board,
            pregen_attacks: PregenAttacks::init(),
            zobrist,
            transposition_table: HashMap::with_capacity(1024),
            history: Vec::with_capacity(256),
            zobrist_history: vec![initial_hash],
            clock_time: time_limit,
            moves_to_go,
        }
    }

    /// Returns a reference to the current board state
    #[inline(always)]
    pub fn get_board_state(&self) -> &BoardState {
        &self.board_state
    }

    /// Returns a reference to the pre-generated attacks
    #[inline(always)]
    pub fn get_pregen_attacks(&self) -> &PregenAttacks {
        &self.pregen_attacks
    }

    /// Transposition table lookup: returns Some(&TTEntry) if present
    pub fn tt_lookup(&self, key: u64) -> Option<&TTEntry> {
        self.transposition_table.get(&key)
    }

    /// Transposition table insert or update
    pub fn tt_insert(&mut self, key: u64, entry: TTEntry) {
        self.transposition_table.insert(key, entry);
    }

    /// Returns the current number of moves to go
    #[inline]
    pub fn get_moves_to_go(&self) -> u32 {
        self.moves_to_go
    }

    /// Decrements the number of moves to go (minimum 1)
    #[inline]
    pub fn decrement_moves_to_go(&mut self) {
        if self.moves_to_go > 1 {
            self.moves_to_go -= 1;
        }
    }

    #[inline]
    pub fn get_clock_time(&self) -> std::time::Duration {
        self.clock_time
    }

    #[inline]
    pub fn decrement_clock_time(&mut self, decrement: u64) {
        self.clock_time = self.clock_time.saturating_sub(std::time::Duration::from_secs(decrement));
    }

    /// Makes a move on the board, recording history and zobrist hash for undo and repetition
    #[inline]
    pub fn make_move(&mut self, m: Move) {
        // Save previous board for undo
        self.history.push(self.board_state.clone());
        // Apply move, updating board_state and internal hash
        self.board_state.make_move(m, &self.zobrist);
        // Record new zobrist hash for repetition detection
        let h = self.board_state.get_zobrist_hash();
        self.zobrist_history.push(h);
    }

    /// Makes a null move (used for null move pruning). Does NOT affect repetition tracking.
    #[inline]
    pub fn make_null_move(&mut self) {
        self.board_state.make_null_move(&self.zobrist);
        // Note: do not push to history or zobrist_history
    }

    /// Unmakes a null move. Does NOT affect repetition tracking.
    #[inline]
    pub fn unmake_null_move(&mut self) {
        self.board_state.unmake_null_move(&self.zobrist);
    }

    /// Unmakes the last move, restoring board state and repetition history
    #[inline]
    pub fn unmake_move(&mut self) {
        // Pop repetition history
        if self.zobrist_history.pop().is_none() {
            panic!("unmake_move: zobrist_history empty");
        }
        // Restore previous board state
        if let Some(prev_state) = self.history.pop() {
            self.board_state = prev_state;
        } else {
            panic!("GameState::unmake_move called with empty history");
        }
    }

    /// Checks if the given side is in check
    #[inline]
    pub fn is_check(&self, side: Color) -> bool {
        self.board_state.is_check(side, &self.pregen_attacks)
    }

    /// Checks if the current position is checkmate
    #[inline]
    pub fn is_checkmate(&self) -> bool {
        let side = self.board_state.get_side();
        // If not in check, can't be checkmate
        if !self.is_check(side) {
            return false;
        }
        // Generate pseudo-legal moves
        let mut moves = Vec::with_capacity(256);
        pseudo_move_gen::get_pseudo_moves(&self.board_state, &self.pregen_attacks, &mut moves);
        // Test each move: if any gets out of check, not checkmate
        for &m in &moves {
            let mut temp = self.board_state.clone();
            temp.make_move(m, &self.zobrist);
            if !temp.is_check(side, &self.pregen_attacks) {
                return false;
            }
        }
        true
    }

    /// Checks if the current position is stalemate
    #[inline]
    pub fn is_stalemate(&self) -> bool {
        let side = self.board_state.get_side();
        // If in check, not stalemate
        if self.is_check(side) {
            return false;
        }
        let mut moves = Vec::with_capacity(256);
        pseudo_move_gen::get_pseudo_moves(&self.board_state, &self.pregen_attacks, &mut moves);
        for &m in &moves {
            let mut temp = self.board_state.clone();
            temp.make_move(m, &self.zobrist);
            if !temp.is_check(side, &self.pregen_attacks) {
                return false;
            }
        }
        true
    }

    /// Checks for threefold repetition draw: current position hash appears at least 3 times
    pub fn is_repetition_draw(&self) -> bool {
        let current = self.board_state.get_zobrist_hash();
        let count = self.zobrist_history.iter().filter(|&&h| h == current).count();
        count >= 3
    }

    /// Returns the half-move clock for fifty-move rule, delegating to BoardState
    pub fn half_move_clock(&self) -> u8 {
        self.board_state.get_half_moves()
    }

    /// Makes the engine's move and returns true if the game should continue
    fn make_engine_move(&mut self) -> bool {
        let now = std::time::Instant::now();
        if let Some(engine_move) = move_eval::find_best_move(self, self.get_clock_time()) {
            let time_elapsed = now.elapsed().as_secs();

            self.decrement_clock_time(time_elapsed);

            if self.get_clock_time().as_secs() <= 0 {
                print!("Times Up");
                return false;
            }

            println!("Time: {}s", time_elapsed);
            println!("Move Selected: {:?}{:?}", engine_move.get_from(), engine_move.get_to());

            self.make_move(engine_move);
            self.board_state.display_info(&self.pregen_attacks);
            self.decrement_moves_to_go();

            // Check for end conditions
            if self.is_checkmate() {
                println!("Checkmate!");
                return false;
            }
            if self.is_stalemate() {
                println!("Stalemate!");
                return false;
            }
            if self.is_repetition_draw() {
                println!("Draw by repetition!");
                return false;
            }
            if self.half_move_clock() >= 100 {
                println!("Draw by fifty-move rule!");
                return false;
            }
            true
        } else {
            println!("Time: {}s", now.elapsed().as_secs());
            println!("No move found");
            false
        }
    }

    /// Makes the player's move and returns true if the game should continue
    fn make_player_move(&mut self) -> bool {
        let user_move = user_input::get_user_move(self);
        self.make_move(user_move);
        self.board_state.display_info(&self.pregen_attacks);
        self.decrement_moves_to_go();

        // Check for end conditions
        if self.is_checkmate() {
            println!("Checkmate!");
            return false;
        }
        if self.is_stalemate() {
            println!("Stalemate!");
            return false;
        }
        if self.is_repetition_draw() {
            println!("Draw by repetition!");
            return false;
        }
        if self.half_move_clock() >= 100 {
            println!("Draw by fifty-move rule!");
            return false;
        }
        true
    }

    /// Runs the main game loop
    pub fn run(&mut self) {
        // Ask who goes first
        println!("Who goes first? (1 for engine, 2 for player)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let engine_first = input.trim() == "1";

        self.board_state.display_info(&self.pregen_attacks);

        loop {
            if engine_first {
                if !self.make_engine_move() {
                    break;
                }
                if !self.make_player_move() {
                    break;
                }
            } else {
                if !self.make_player_move() {
                    break;
                }
                if !self.make_engine_move() {
                    break;
                }
            }
        }
    }
}
