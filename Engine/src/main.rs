use move_gen::move_eval;

use crate::core::attack_pregen;
use crate::core::bitboard::Bitboard;
use crate::core::piece::*;
use crate::core::square::*;
use crate::move_gen::move_encode::Move;

mod core;
mod game;
mod move_gen;
fn main() {
    let mut game = game::Game::new(Some("rnbqk2r/pppppppp/8/8/8/8/8/3QK3 b kq - 0 1"));

    game.run();

    // print!("{:?}", Move::move_from_algebraic("Kb1c3", Color::White).get_from());

    // print!("{:?}", -(-(std::i32::MIN + 1)))

    // let sudo_moves = move_gen::move_gen::get_sudo_moves(&mut game);

    // sudo_moves.iter().for_each(|m| {
    //     if m.get_piece().get_piece_type() == PieceType::King {
    //         m.print_move();
    //     }
    // });
}
