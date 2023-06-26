use crate::core::attack_pregen;
use crate::core::bitboard::Bitboard;
use crate::core::piece::*;
use crate::core::square::*;

mod core;
mod game;
mod move_gen;
fn main() {
    let mut game = game::Game::new(Some("rnb1kbnr/pppp1ppp/4p3/8/P7/4P3/1PP1B1P1/RNBQK1Nq w - - 0 1"));

    game.get_board_state().display_info();

    // println!(
    //     "{}",
    //     game.get_pregen_attacks()
    //         .get_queen_attacks(Square::E1, game.get_board_state().get_position_bb(Color::Both))
    // )

    for x in move_gen::move_gen::gen_queen_moves(&game.get_board_state(), &game.get_pregen_attacks()) {
        x.print_move();
    }

    // game.get_pregen_attacks()
    //     .get_bishop_attacks(Square::E2, game.get_board_state().get_position_bb(Color::Both))
    //     .print_bb();
}
