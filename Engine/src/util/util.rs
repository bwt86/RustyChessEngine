//Utility File
use std::ops::{Index, IndexMut, Shl};

use crate::board::square::Square;

//Checks if a square is occupied
pub fn is_occupied(bit_board: u64, square: Square) -> bool {
    return bit_board & (1 << square as u64) != 0;
}

//Prints out bit_board unformated
pub fn print_bb(bit_board: u64) {
    print!("\n");
    for rank in (0..8).rev() {
        for file in 0..8 {
            if is_occupied(bit_board, Square::get_square(rank, file)) {
                print!("| 1 |");
            } else {
                print!("| 0 |");
            }
        }
        print!("\n");
    }
    print!("\n");
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
