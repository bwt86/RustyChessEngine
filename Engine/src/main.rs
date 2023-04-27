use board_info::{board::Board, square::Square};
use move_gen::attacks::PregenAttacks;
use util::{
    bit_masks::{FILE_BB, RANKS_BB},
    util::{get_line_east, get_line_north, get_line_south, get_line_west, print_bb},
};

mod board_info;
mod move_gen;
mod util;

fn main() {
    // let b = Board::init(None);

    // b.display_info();

    let a = PregenAttacks::init();

    print_bb(a.get_rook_attacks(Square::D1, Square::D7.to_bit_board()));
}
