use board::board::Board;
use move_gen::attacks::Pregen_Attacks;
use util::{
    bit_masks::{FILE_A, FILE_H, RANK_1, RANK_8},
    util::{get_line_north, print_bb},
};

mod board;
mod move_gen;
mod util;

fn main() {
    let b = Board::init(None);

    b.display_info();

    let a = Pregen_Attacks::init();

    print_bb(a.get_bishop_attacks(board::square::Square::D4, 0u64));
}
