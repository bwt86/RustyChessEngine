use crate::core::attack_pregen;
use crate::core::bitboard::Bitboard;
use crate::core::piece::*;
use crate::core::square::*;

mod core;
mod game;
mod move_gen;
fn main() {
    let mut game = game::Game::new(Some("rnb1kbnr/pppp1ppp/4p3/8/2P4q/1P6/P2PPPPP/RNBQKBNR w KQkq - 0 1"));

    game.get_board_state().display_info();

    // println!(
    //     "{}",
    //     game.get_pregen_attacks()
    //         .get_queen_attacks(Square::E1, game.get_board_state().get_position_bb(Color::Both))
    // )

    for x in move_gen::move_gen::gen_pawn_moves(&game.get_board_state(), &game.get_pregen_attacks()) {
        x.print_move();
    }
}
