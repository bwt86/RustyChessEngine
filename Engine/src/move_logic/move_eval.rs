use crate::game_logic::game::{GameState, TTEntry};
use std::time::{Duration, Instant};

use super::{move_encode::Move, pseudo_move_gen};

// Search constants
const NULL_MOVE_DEPTH: u8 = 3;
const NULL_MOVE_REDUCTION: u8 = 2;
const NULL_MOVE_PIECE_COUNT: u8 = 8;
const ASPIRATION_WINDOW: i32 = 50;
const FUTILITY_MARGIN: i32 = 150;
const LMR_MIN_DEPTH: u8 = 3;
const LMR_MIN_MOVE: usize = 4;
const QUIESCENCE_DEPTH: u8 = 6;
const MATE_SCORE: i32 = 100_000;
const DRAW_SCORE: i32 = 0;
const MAX_PLY: u8 = 64;
const RAZOR_MARGIN: i32 = 300;
const DELTA_MARGIN: i32 = 975;

// Transposition table entry types
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EntryType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Debug)]
struct SearchInfo {
    start_time: Instant,
    nodes: u64,
    best_move: Option<Move>,
    best_score: i32,
    depth: u8,
    pv: Vec<Move>, // principal variation from root
    selective_depth: u8,
    time_management: TimeManagement,
    ply: u8,
    stop: bool,
}

#[derive(Clone, Debug)]
struct TimeManagement {
    max_time: Duration,
    optimal_time: Duration,
    min_time: Duration,
    max_nodes: u64,
}

impl TimeManagement {
    fn new(time_left: Duration, increment: Duration, moves_to_go: u32) -> Self {
        let moves_to_go = moves_to_go.max(1);

        // Calculate optimal time based on game phase
        let optimal_time = if moves_to_go <= 5 {
            // Endgame: use more time
            time_left / moves_to_go
        } else {
            // Opening/middlegame: use less time
            time_left / (moves_to_go * 2)
        };

        // Add increment if available
        let optimal_time = optimal_time + increment;

        // Set minimum time to avoid instant moves
        let min_time = Duration::from_millis(100);

        // Set maximum time to avoid time trouble
        let max_time = optimal_time * 2;

        // Estimate max nodes based on time
        let max_nodes = 1_000_000; // Conservative estimate

        Self {
            max_time,
            optimal_time,
            min_time,
            max_nodes,
        }
    }

    fn should_stop(&self, elapsed: Duration, nodes: u64) -> bool {
        // Stop if we've exceeded max time
        if elapsed >= self.max_time {
            return true;
        }

        // Stop if we've searched enough nodes
        if nodes >= self.max_nodes {
            return true;
        }

        // Don't stop if we haven't searched minimum time
        if elapsed < self.min_time {
            return false;
        }

        // Stop if we've used more than optimal time
        elapsed > self.optimal_time
    }
}

impl SearchInfo {
    fn new(time_limit: Duration, moves_to_go: u32) -> Self {
        Self {
            start_time: Instant::now(),
            nodes: 0,
            best_move: None,
            best_score: i32::MIN + 1,
            depth: 0,
            pv: Vec::with_capacity(MAX_PLY as usize),
            selective_depth: 0,
            time_management: TimeManagement::new(time_limit, Duration::from_secs(0), moves_to_go),
            ply: 0,
            stop: false,
        }
    }

    fn time_elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    fn should_stop(&self) -> bool {
        self.time_management.should_stop(self.time_elapsed(), self.nodes)
    }

    fn update_selective_depth(&mut self, depth: u8) {
        self.selective_depth = self.selective_depth.max(depth);
    }
}

/// Find the best move within the given time limit. keeps prints for info.
pub fn find_best_move(game_state: &mut GameState, time_limit: Duration) -> Option<Move> {
    let moves_to_go = game_state.get_moves_to_go().max(1);
    let mut search_info = SearchInfo::new(time_limit, moves_to_go);
    let mut window_size = ASPIRATION_WINDOW;
    let mut depth = 1;
    let mut last_full_best_move = None;
    let mut last_full_pv: Vec<Move> = Vec::new();
    let mut last_full_score = 0;

    while depth <= MAX_PLY {
        search_info.depth = depth;
        search_info.pv.clear();
        search_info.stop = false;

        let mut alpha = i32::MIN + 1;
        let mut beta = i32::MAX - 1;
        // Aspiration window
        if depth > 4 && last_full_best_move.is_some() {
            alpha = last_full_score - window_size;
            beta = last_full_score + window_size;
        }

        // First search
        let (mut score, pv) = negamax(game_state, depth, alpha, beta, false, &mut search_info);
        // If time ran out during search, break without updating last full
        if search_info.stop {
            break;
        }
        // If fail-low or fail-high, re-search with full window
        if score <= alpha || score >= beta {
            let (rescore, full_pv) = negamax(game_state, depth, i32::MIN + 1, i32::MAX - 1, false, &mut search_info);
            if search_info.stop {
                break;
            }
            score = rescore;
            // Use full window PV
            search_info.pv = full_pv.clone();
        } else {
            // Use pv from first search
            search_info.pv = pv.clone();
        }

        // Completed this depth fully
        last_full_best_move = search_info.best_move;
        last_full_pv = search_info.pv.clone();
        last_full_score = score;

        // Adjust window for next iteration
        if score <= alpha {
            window_size = window_size * 2;
        } else if score >= beta {
            window_size = window_size * 2;
        } else {
            window_size = (window_size * 3) / 2;
        }

        depth += 1;
    }

    // Print search information for this depth
    println!("Search completed at depth {}:", depth);
    println!("  Depth: {}", search_info.depth);
    println!("  Selective depth: {}", search_info.selective_depth);
    println!("  Nodes searched: {}", search_info.nodes);
    println!("  Time elapsed: {:?}", search_info.time_elapsed());
    println!("  Best score: {}", last_full_score);
    println!("  Moves to go: {}", game_state.get_moves_to_go());
    println!("  Principal Variation:");
    for (i, mv) in last_full_pv.iter().enumerate() {
        println!("    {}. {:?}", i + 1, mv);
    }
    if let Some(best_move) = last_full_best_move {
        println!("  Best move: {:?}", best_move);
    }

    // Update moves_to_go in game state after finding the best move
    game_state.decrement_moves_to_go();
    last_full_best_move
}

/// Negamax search returning (score, PV moves from this node)
fn negamax(game_state: &mut GameState, depth: u8, mut alpha: i32, mut beta: i32, null_move: bool, search_info: &mut SearchInfo) -> (i32, Vec<Move>) {
    // Time check
    if search_info.should_stop() {
        search_info.stop = true;
        return (0, Vec::new());
    }
    search_info.nodes += 1;
    let current_ply = search_info.depth.saturating_sub(depth);
    search_info.update_selective_depth(current_ply);

    // Terminal checks
    if game_state.is_checkmate() {
        return (-MATE_SCORE + current_ply as i32, Vec::new());
    }
    if game_state.is_stalemate() {
        return (DRAW_SCORE, Vec::new());
    }
    if game_state.is_repetition_draw() {
        return (DRAW_SCORE, Vec::new());
    }
    if game_state.half_move_clock() >= 100 {
        return (DRAW_SCORE, Vec::new());
    }

    // Razor pruning
    if depth == 1 && !game_state.is_check(game_state.get_board_state().get_side()) {
        let stand_pat = game_state.get_board_state().evaluate(game_state.get_pregen_attacks());
        if stand_pat + RAZOR_MARGIN < alpha {
            return (alpha, Vec::new());
        }
    }

    // Transposition Table lookup using new interface
    let zobrist_key = game_state.get_board_state().get_zobrist_hash();
    let orig_alpha = alpha;
    let orig_beta = beta;
    if let Some(entry) = game_state.tt_lookup(zobrist_key) {
        let tt_score = entry.score;
        let tt_depth = entry.depth;
        let tt_entry_type = entry.entry_type;
        if tt_depth >= depth {
            match tt_entry_type {
                EntryType::Exact => return (tt_score, Vec::new()),
                EntryType::LowerBound => {
                    alpha = alpha.max(tt_score);
                }
                EntryType::UpperBound => {
                    beta = beta.min(tt_score);
                }
            }
            if alpha >= beta {
                return (tt_score, Vec::new());
            }
        }
    }

    // Null move pruning
    if !null_move
        && depth >= NULL_MOVE_DEPTH
        && game_state.get_board_state().get_num_pieces() > NULL_MOVE_PIECE_COUNT
        && !game_state.is_check(game_state.get_board_state().get_side())
    {
        game_state.make_null_move();
        search_info.ply += 1;
        let (score, _) = negamax(game_state, depth - 1 - NULL_MOVE_REDUCTION, -beta, -alpha, true, search_info);
        search_info.ply -= 1;
        game_state.unmake_null_move();
        if search_info.stop {
            return (0, Vec::new());
        }
        if -score >= beta {
            return (beta, Vec::new());
        }
    }

    // Leaf node: quiescence
    if depth == 0 {
        let score = quiescence_search(game_state, alpha, beta, search_info);
        return (score, Vec::new());
    }

    // Generate moves
    let mut pseudo_moves: Vec<Move> = Vec::with_capacity(256);
    pseudo_move_gen::get_pseudo_moves(game_state.get_board_state(), game_state.get_pregen_attacks(), &mut pseudo_moves);
    order_moves(&mut pseudo_moves);

    let mut best_score = i32::MIN + 1;
    let mut best_pv: Vec<Move> = Vec::new();
    let mut moves_searched = 0;
    let mut pv_found = false;

    for m in pseudo_moves {
        if search_info.should_stop() {
            search_info.stop = true;
            break;
        }

        // Delta pruning
        if depth == 1 && !pv_found {
            let captured_value = m.get_capture().map_or(0, |p| p.get_value());
            if captured_value + DELTA_MARGIN < alpha {
                continue;
            }
        }

        game_state.make_move(m);
        search_info.ply += 1;

        // Skip illegal moves
        if game_state.is_check(game_state.get_board_state().get_opposite_side()) {
            search_info.ply -= 1;
            game_state.unmake_move();
            continue;
        }

        let (score, child_pv) = if !pv_found {
            // full window first
            let (s, pv) = negamax(game_state, depth - 1, -beta, -alpha, false, search_info);
            (s, pv)
        } else {
            // Late Move Reduction with improved conditions
            let reduced_depth = if depth >= LMR_MIN_DEPTH
                && moves_searched >= LMR_MIN_MOVE
                && !m.is_capture()
                && !game_state.is_check(game_state.get_board_state().get_side())
            {
                depth.saturating_sub(2)
            } else {
                depth - 1
            };
            let (s_reduced, pv_reduced) = negamax(game_state, reduced_depth, -(alpha + 1), -alpha, false, search_info);
            if search_info.stop {
                search_info.ply -= 1;
                game_state.unmake_move();
                break;
            }
            let mut s = s_reduced;
            let mut pv = pv_reduced;
            if s > alpha && reduced_depth < depth - 1 {
                let (s_full, pv_full) = negamax(game_state, depth - 1, -beta, -alpha, false, search_info);
                if search_info.stop {
                    search_info.ply -= 1;
                    game_state.unmake_move();
                    break;
                }
                s = s_full;
                pv = pv_full;
            }
            (s, pv)
        };

        let score = -score;
        search_info.ply -= 1;
        game_state.unmake_move();
        if search_info.stop {
            break;
        }

        if score > best_score {
            best_score = score;
            // Update alpha and PV
            if score > alpha {
                alpha = score;
                pv_found = true;
                // Build PV: current move + child PV
                best_pv.clear();
                best_pv.push(m);
                // child_pv contains moves from child node; append
                for &mv in &child_pv {
                    best_pv.push(mv);
                }
                // If at root, update global best_move
                if current_ply == 0 {
                    search_info.best_move = Some(m);
                    search_info.best_score = score;
                }
            }
        }
        if alpha >= beta {
            break;
        }
        moves_searched += 1;
    }

    // Store in TT using new interface
    if !search_info.stop {
        let entry_type = if best_score <= orig_alpha {
            EntryType::UpperBound
        } else if best_score >= orig_beta {
            EntryType::LowerBound
        } else {
            EntryType::Exact
        };
        let entry = TTEntry {
            score: best_score,
            depth,
            entry_type,
        };
        game_state.tt_insert(zobrist_key, entry);
    }

    (alpha, best_pv)
}

/// Quiescence search returning a static score. PV not tracked here.
fn quiescence_search(game_state: &mut GameState, mut alpha: i32, beta: i32, search_info: &mut SearchInfo) -> i32 {
    if search_info.should_stop() {
        search_info.stop = true;
        return alpha;
    }
    search_info.nodes += 1;
    // Optionally update selective depth deeper
    search_info.update_selective_depth(search_info.depth);

    let stand_pat = game_state.get_board_state().evaluate(game_state.get_pregen_attacks());
    if stand_pat >= beta {
        return beta;
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    // Add depth check
    if search_info.ply >= QUIESCENCE_DEPTH {
        return alpha;
    }

    let mut pseudo_moves: Vec<Move> = Vec::with_capacity(256);
    pseudo_move_gen::get_pseudo_moves(game_state.get_board_state(), game_state.get_pregen_attacks(), &mut pseudo_moves);
    // Only captures
    pseudo_moves.retain(|m| m.is_capture());
    order_moves(&mut pseudo_moves);

    for m in pseudo_moves {
        if search_info.should_stop() {
            search_info.stop = true;
            break;
        }
        let captured_value = m.get_capture().map_or(0, |p| p.get_value());
        if stand_pat + captured_value + FUTILITY_MARGIN < alpha {
            continue;
        }
        game_state.make_move(m);
        search_info.ply += 1;
        if game_state.is_check(game_state.get_board_state().get_opposite_side()) {
            search_info.ply -= 1;
            game_state.unmake_move();
            continue;
        }
        let score = -quiescence_search(game_state, -beta, -alpha, search_info);
        search_info.ply -= 1;
        game_state.unmake_move();
        if search_info.stop {
            break;
        }
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

/// Simple move ordering by stored move scores; can be extended with history, killer heuristics, etc.
pub fn order_moves(moves: &mut Vec<Move>) {
    moves.sort_by(|a, b| b.get_score().cmp(&a.get_score()));
}
