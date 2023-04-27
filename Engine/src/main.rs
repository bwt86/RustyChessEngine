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

    a.print_bishop_masks();
    print!("--------------------------------------");
    a.print_rook_masks();
}
