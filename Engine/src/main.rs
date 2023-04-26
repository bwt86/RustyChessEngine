use board_info::board::Board;
use move_gen::attacks::PregenAttacks;
use util::{
    bit_masks::{FILE_A, FILE_H, RANK_1, RANK_8},
    util::{get_line_north, print_bb},
};

mod board_info;
mod move_gen;
mod util;

fn main() {
    let b = Board::init(None);

    b.display_info();

    let a = PregenAttacks::init();

    print_bb(a.get_bishop_attacks(board_info::square::Square::D4, 0u64));
}
