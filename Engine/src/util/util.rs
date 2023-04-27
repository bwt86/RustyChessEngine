use std::str::Bytes;

//Utility File
use crate::board_info::{
    piece::PIECE_CHARS,
    square::{get_square, Square},
};

use super::bit_masks::{FILE_BB, RANKS_BB};

//Checks if a square is occupied
pub fn is_occupied(bit_board: u64, square: Square) -> bool {
    return bit_board & (1 << square as u64) != 0;
}

//Prints out bit_board unformated
pub fn print_bb(bit_board: u64) {
    println!();
    for rank in (0..8).rev() {
        for file in 0..8 {
            if is_occupied(bit_board, get_square(rank, file)) {
                print!("| 1 |");
            } else {
                print!("| 0 |");
            }
        }
        println!();
    }
    println!();
}

pub fn set_bit(bit_board: &mut u64, square: Square) {
    *bit_board |= 1 << square;
}

//Returns the number of bits in a u64.
pub fn count_bits(bit_board: u64) -> u8 {
    return bit_board.count_ones() as u8;
}

//Returns the least significant bit of a u64.
pub fn get_lsb(bit_board: u64) -> u64 {
    return 1_u64.wrapping_shl(bit_board.trailing_zeros());
}

//Removes the least significant bit of a u64 and returns it.
pub fn pop_lsb(bit_board: &mut u64) -> u64 {
    let bit = get_lsb(*bit_board);
    *bit_board ^= bit;
    return bit;
}

//Returns the most significant bit of a u64.
pub fn get_msb(bit_board: u64) -> u64 {
    return 1_u64.wrapping_shl(63 - bit_board.leading_zeros());
}

//Removes the most significant bit of a u64 and returns it.
pub fn pop_msb(bit_board: &mut u64) -> u64 {
    let bit = get_msb(*bit_board);
    *bit_board ^= bit;
    return bit;
}

//Returns the index of the least significant bit of a u64.
pub fn bit_scan_forward(bit_board: u64) -> u8 {
    return bit_board.trailing_zeros() as u8;
}

pub fn get_line_north(square: Square) -> u64 {
    let ray: u64 = FILE_BB[square.get_file() as usize];
    let ranks: u64 = RANKS_BB[square.get_rank() as usize..].iter().sum();

    return ray & ranks & !square.to_bit_board();
}

pub fn get_line_south(square: Square) -> u64 {
    let ray: u64 = FILE_BB[square.get_file() as usize];
    let ranks: u64 = RANKS_BB[square.get_rank() as usize..].iter().sum();

    return ray & !ranks & !square.to_bit_board();
}

pub fn get_line_east(square: Square) -> u64 {
    let ray: u64 = RANKS_BB[square.get_rank() as usize];
    let files: u64 = FILE_BB[square.get_file() as usize..].iter().sum();

    return ray & files & !square.to_bit_board();
}

pub fn get_line_west(square: Square) -> u64 {
    let ray: u64 = RANKS_BB[square.get_rank() as usize];
    let files: u64 = FILE_BB[square.get_file() as usize..].iter().sum();

    return ray & !files & !square.to_bit_board();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_cust_bitshift() {
        assert_eq!(1 << Square::A1, 1);
        assert_eq!(1 << Square::B1, 2);
        assert_eq!(1 << Square::C1, 4);
    }

    #[test]
    fn test_bit_op() {
        let bit_board: u64 = 0b110011010101110000011100010101;
        assert_eq!(count_bits(bit_board), 15);
        assert_eq!(get_lsb(bit_board), 1);
        assert_eq!(get_msb(bit_board), 0b100000000000000000000000000000);
        assert_eq!(bit_scan_forward(bit_board), 0);
    }
}
