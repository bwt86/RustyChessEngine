use crate::core::square::{FILES, RANKS};

use super::square::Square;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    // Initialize a bitboard with a given u64
    pub fn init(board: u64) -> Self {
        Bitboard(board)
    }

    pub fn from_square(sq: Square) -> Self {
        Bitboard(1 << sq)
    }

    pub fn get_board(&self) -> u64 {
        self.0
    }

    // Get bit at a given square
    pub fn get_bit(&self, sq: Square) -> u8 {
        ((self.0 >> sq) & 1) as u8
    }

    // Set a bit at a given square
    pub fn set_bit(&mut self, sq: Square) {
        self.0 |= 1 << sq;
    }

    // Clear a bit at a given square
    pub fn clear_bit(&mut self, sq: Square) {
        self.0 &= !(1 << sq);
    }

    pub fn make_move(&mut self, from: Square, to: Square) {
        self.clear_bit(from);
        self.set_bit(to);
    }

    // Check if a bit is set at a given square
    pub fn is_occupied(&self, sq: Square) -> bool {
        self.0 & (1 << sq) != 0
    }

    // Count the number of bits set in a bitboard
    pub fn count_bits(&self) -> u8 {
        self.0.count_ones() as u8
    }

    // Get the least significant bit from a bitboard
    pub fn get_lsb(&self) -> Bitboard {
        Bitboard(self.0 & self.0.wrapping_neg())
    }

    // Pop the least significant bit from a bitboard
    pub fn pop_lsb(&mut self) -> Bitboard {
        let bit = self.get_lsb();
        self.0 &= !bit.0;
        bit
    }

    // Get the most significant bit from a bitboard
    pub fn get_msb(&self) -> Bitboard {
        Bitboard(1_u64.wrapping_shl(63 - self.0.leading_zeros()))
    }

    // Pop the most significant bit from a bitboard
    pub fn pop_msb(&mut self) -> Bitboard {
        let bit = self.get_msb();
        self.0 &= !bit.0;
        bit
    }

    // Get the index of the least significant bit
    pub fn bit_scan_forward(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    // print a bitboard to the console as 1's and 0's
    pub fn print_bb(&self) {
        println!();
        for rank in RANKS.iter().rev() {
            for file in FILES {
                print!("| {} |", self.get_bit(Square::from_rank_file(*rank, file)))
            }
            println!();
        }
        println!();
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl<T> std::ops::Shl<T> for Bitboard
where
    u64: std::ops::Shl<T, Output = u64>,
{
    type Output = Self;

    fn shl(self, rhs: T) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl<T> std::ops::Shr<T> for Bitboard
where
    u64: std::ops::Shr<T, Output = u64>,
{
    type Output = Self;

    fn shr(self, rhs: T) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl<T> std::ops::Sub<T> for Bitboard
where
    u64: std::ops::Sub<T, Output = u64>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Bitboard(self.0 - rhs)
    }
}

impl std::ops::Sub<Bitboard> for Bitboard {
    type Output = Self;

    fn sub(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 - rhs.0)
    }
}

impl<T> std::ops::Add<T> for Bitboard
where
    u64: std::ops::Add<T, Output = u64>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Bitboard(self.0 + rhs)
    }
}

impl<T> std::ops::Mul<T> for Bitboard
where
    u64: std::ops::Mul<T, Output = u64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Bitboard(self.0 * rhs)
    }
}

impl PartialEq<u64> for Bitboard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_init() {
        let bb = Bitboard::init(1024);
        assert_eq!(bb.0, 1024);
    }

    #[test]
    fn test_bitboard_set_bit() {
        let mut bb = Bitboard::init(0);
        bb.set_bit(Square::D4);
        assert_eq!(bb.0, 1 << Square::D4);
    }

    #[test]
    fn test_bitboard_get_bit() {
        let mut bb = Bitboard::init(0);
        bb.set_bit(Square::E5);
        assert_eq!(bb.get_bit(Square::E5), 1);
        assert_eq!(bb.get_bit(Square::D4), 0);
    }

    #[test]
    fn test_bitboard_clear_bit() {
        let mut bb = Bitboard::init(0);
        bb.set_bit(Square::E5);
        bb.clear_bit(Square::E5);
        assert_eq!(bb.get_bit(Square::E5), 0);
    }

    #[test]
    fn test_bitboard_is_occupied() {
        let mut bb = Bitboard::init(0);
        bb.set_bit(Square::E5);
        assert_eq!(bb.is_occupied(Square::E5), true);
        assert_eq!(bb.is_occupied(Square::D4), false);
    }

    #[test]
    fn test_bitboard_count_bits() {
        let mut bb = Bitboard::init(0);
        bb.set_bit(Square::E5);
        bb.set_bit(Square::D4);
        assert_eq!(bb.count_bits(), 2);
    }

    #[test]
    fn test_bitboard_get_lsb() {
        let mut bb = Bitboard::init(0);
        bb.set_bit(Square::E5);
        bb.set_bit(Square::D4);
        assert_eq!(bb.get_lsb(), Bitboard::from_square(Square::D4));
    }

    #[test]
    fn test_bitboard_pop_lsb() {
        let mut bb = Bitboard::init(0);
        bb.set_bit(Square::E5);
        bb.set_bit(Square::D4);
        assert_eq!(bb.pop_lsb(), Bitboard::from_square(Square::D4));
        assert_eq!(bb.is_occupied(Square::D4), false);
    }

    #[test]
    fn test_bitboard_get_msb() {
        let mut bb = Bitboard::init(0);
        bb.set_bit(Square::E5);
        bb.set_bit(Square::D4);
        assert_eq!(bb.get_msb(), Bitboard::from_square(Square::E5));
    }

    #[test]
    fn test_bitboard_pop_msb() {
        let mut bb = Bitboard::init(0);
        bb.set_bit(Square::E5);
        bb.set_bit(Square::D4);
        assert_eq!(bb.pop_msb(), Bitboard::from_square(Square::E5));
        assert_eq!(bb.is_occupied(Square::E5), false);
    }

    #[test]
    fn test_bitboard_bit_scan_forward() {
        let mut bb = Bitboard::init(0);
        bb.set_bit(Square::E5);
        bb.set_bit(Square::D4);
        assert_eq!(bb.bit_scan_forward(), Square::D4 as u8);
    }
}
