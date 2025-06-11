use crate::core::square::*;

use super::piece::Color;

/// A bitboard representing a set of squares on a chess board.
/// Each bit represents a square, with the least significant bit being A1.
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct Bitboard(pub u64);

//Bit board of each file
pub const FILE_A_BB: Bitboard = Bitboard(0x0101010101010101);
pub const FILE_B_BB: Bitboard = Bitboard(0x0202020202020202);
pub const FILE_C_BB: Bitboard = Bitboard(0x0404040404040404);
pub const FILE_D_BB: Bitboard = Bitboard(0x0808080808080808);
pub const FILE_E_BB: Bitboard = Bitboard(0x1010101010101010);
pub const FILE_F_BB: Bitboard = Bitboard(0x2020202020202020);
pub const FILE_G_BB: Bitboard = Bitboard(0x4040404040404040);
pub const FILE_H_BB: Bitboard = Bitboard(0x8080808080808080);

pub const FILES_BB: [Bitboard; 8] = [FILE_A_BB, FILE_B_BB, FILE_C_BB, FILE_D_BB, FILE_E_BB, FILE_F_BB, FILE_G_BB, FILE_H_BB];

//Bit board of each rank
pub const RANK_1_BB: Bitboard = Bitboard(0x00000000000000ff);
pub const RANK_2_BB: Bitboard = Bitboard(0x000000000000ff00);
pub const RANK_3_BB: Bitboard = Bitboard(0x0000000000ff0000);
pub const RANK_4_BB: Bitboard = Bitboard(0x00000000ff000000);
pub const RANK_5_BB: Bitboard = Bitboard(0x000000ff00000000);
pub const RANK_6_BB: Bitboard = Bitboard(0x0000ff0000000000);
pub const RANK_7_BB: Bitboard = Bitboard(0x00ff000000000000);
pub const RANK_8_BB: Bitboard = Bitboard(0xff00000000000000);

pub const RANKS_BB: [Bitboard; 8] = [RANK_1_BB, RANK_2_BB, RANK_3_BB, RANK_4_BB, RANK_5_BB, RANK_6_BB, RANK_7_BB, RANK_8_BB];

pub const DARK_SQUARES_BB: Bitboard = Bitboard(0xAA55AA55AA55AA55);
pub const LIGHT_SQUARES_BB: Bitboard = Bitboard(0xAA55AA55AA55AA55);

impl Bitboard {
    /// Creates an empty bitboard
    #[inline(always)]
    pub const fn new_empty() -> Self {
        Bitboard(0)
    }

    /// Creates a bitboard with all squares set
    #[inline(always)]
    pub const fn new_full() -> Self {
        Bitboard(0xFFFFFFFFFFFFFFFF)
    }

    /// Creates a bitboard from a u64 value
    #[inline(always)]
    pub const fn new_from_u64(val: u64) -> Self {
        Bitboard(val)
    }

    /// Creates a bitboard with a single square set
    #[inline(always)]
    pub const fn new_from_square(square: Square) -> Self {
        Bitboard(1u64 << square as u64)
    }

    /// Sets a square in the bitboard
    #[inline(always)]
    pub fn set_square(&mut self, square: Square) {
        self.0 |= 1u64 << square as u64;
    }

    /// Clears a square in the bitboard
    #[inline(always)]
    pub fn clear_square(&mut self, square: Square) {
        self.0 &= !(1u64 << square as u64);
    }

    /// Checks if a square is occupied
    #[inline(always)]
    pub const fn is_occupied(&self, square: Square) -> bool {
        (self.0 & (1u64 << square as u64)) != 0
    }

    /// Gets the least significant set square
    #[inline(always)]
    pub const fn get_ls_square(&self) -> Square {
        debug_assert!(!self.is_empty(), "Cannot get lsb of empty bitboard");
        Square::from_index(self.0.trailing_zeros() as usize)
    }

    /// Pops the least significant set square and returns it
    #[inline(always)]
    pub fn pop_ls_square(&mut self) -> Square {
        let lsb = self.get_ls_square();
        self.0 &= self.0 - 1;
        lsb
    }

    /// Gets the most significant set square
    #[inline(always)]
    pub const fn get_ms_square(&self) -> Square {
        debug_assert!(!self.is_empty(), "Cannot get msb of empty bitboard");
        Square::from_index(63 - self.0.leading_zeros() as usize)
    }

    /// Pops the most significant set square and returns it
    #[inline(always)]
    pub fn pop_ms_square(&mut self) -> Square {
        let msb = self.get_ms_square();
        self.0 &= self.0 - 1;
        msb
    }

    /// Counts the number of set squares
    #[inline(always)]
    pub const fn count_squares(&self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Gets all occupied squares as a vector
    #[inline]
    pub fn get_occupied_squares(&self) -> Vec<Square> {
        let mut squares = Vec::with_capacity(self.count_squares() as usize);
        let mut bb = *self;

        while !bb.is_empty() {
            squares.push(bb.pop_ls_square());
        }

        squares
    }

    /// Checks if the bitboard is empty
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn print_bb(&self) {
        println!("{}", self);
    }

    /// Gets the occupied squares in a rank
    #[inline(always)]
    pub fn get_occ_rank(&self, rank: Rank) -> Self {
        self.intersect(RANKS_BB[rank as usize])
    }

    /// Gets the occupied squares in a file
    #[inline(always)]
    pub fn get_occ_file(&self, file: File) -> Self {
        self.intersect(FILES_BB[file as usize])
    }

    /// Gets the magic index for sliding piece move generation
    #[inline(always)]
    pub fn get_magic_index(&self, mask: &Bitboard, magic: &u64, shift: &u64) -> usize {
        ((self.0 & mask.0).wrapping_mul(*magic) >> (64 - shift)) as usize
    }

    /// Combines two bitboards
    #[inline(always)]
    pub fn combine(&self, other: Bitboard) -> Self {
        Bitboard(self.0 | other.0)
    }

    /// Intersects two bitboards
    #[inline(always)]
    pub fn intersect(&self, other: Bitboard) -> Self {
        Bitboard(self.0 & other.0)
    }

    /// Gets the difference between two bitboards
    #[inline(always)]
    pub fn diff(&self, other: Bitboard) -> Self {
        Bitboard(self.0 & !other.0)
    }

    /// Inverts the bitboard
    #[inline(always)]
    pub fn invert(&self) -> Self {
        Bitboard(!self.0)
    }

    /// Makes a move on the bitboard
    #[inline(always)]
    pub fn make_move(&mut self, from: Square, to: Square) {
        self.clear_square(from);
        self.set_square(to);
    }

    /// Shifts the bitboard up
    #[inline(always)]
    pub fn shift_up(&self, num_sq: u8) -> Self {
        Bitboard(self.0 << (8 * num_sq))
    }

    /// Shifts the bitboard down
    #[inline(always)]
    pub fn shift_down(&self, num_sq: u8) -> Self {
        Bitboard(self.0 >> (8 * num_sq))
    }

    /// Shifts the bitboard left
    #[inline(always)]
    pub fn shift_left(&self, num_sq: u8) -> Self {
        Bitboard(self.0 >> num_sq)
    }

    /// Shifts the bitboard right
    #[inline(always)]
    pub fn shift_right(&self, num_sq: u8) -> Self {
        Bitboard(self.0 << num_sq)
    }

    /// Shifts the bitboard up and left
    #[inline(always)]
    pub fn shift_up_left(&self, num_sq: u8) -> Self {
        Bitboard(self.0 << (7 * num_sq)).intersect(!FILE_H_BB)
    }

    /// Shifts the bitboard up and right
    #[inline(always)]
    pub fn shift_up_right(&self, num_sq: u8) -> Self {
        Bitboard(self.0 << (9 * num_sq)).intersect(!FILE_A_BB)
    }

    /// Shifts the bitboard down and left
    #[inline(always)]
    pub fn shift_down_left(&self, num_sq: u8) -> Self {
        Bitboard(self.0 >> (9 * num_sq)).intersect(!FILE_H_BB)
    }

    /// Shifts the bitboard down and right
    #[inline(always)]
    pub fn shift_down_right(&self, num_sq: u8) -> Self {
        Bitboard(self.0 >> (7 * num_sq)).intersect(!FILE_A_BB)
    }

    /// Gets pawn attack squares
    #[inline(always)]
    pub fn shift_pawn_attack(&self, color: Color) -> Self {
        match color {
            Color::White => self.shift_up_left(1).combine(self.shift_up_right(1)),
            Color::Black => self.shift_down_left(1).combine(self.shift_down_right(1)),
        }
    }

    /// Gets knight attack squares
    #[inline(always)]
    pub fn shift_knight_attack(&self) -> Self {
        let ul = self.shift_right(15).intersect(!FILE_H_BB);
        let ur = self.shift_right(17).intersect(!FILE_A_BB);
        let dl = self.shift_left(17).intersect(!FILE_H_BB);
        let dr = self.shift_left(15).intersect(!FILE_A_BB);
        let lu = self.shift_right(6).intersect(!(FILE_H_BB.combine(FILE_G_BB)));
        let ld = self.shift_left(10).intersect(!(FILE_H_BB.combine(FILE_G_BB)));
        let ru = self.shift_right(10).intersect(!(FILE_A_BB.combine(FILE_B_BB)));
        let rd = self.shift_left(6).intersect(!(FILE_A_BB.combine(FILE_B_BB)));

        ul.combine(ur).combine(dl).combine(dr).combine(lu).combine(ld).combine(ru).combine(rd)
    }

    /// Gets king attack squares
    #[inline(always)]
    pub fn shift_king_attack(&self) -> Self {
        let up = self.shift_up(1);
        let down = self.shift_down(1);
        let left = self.shift_left(1).intersect(!FILE_H_BB);
        let right = self.shift_right(1).intersect(!FILE_A_BB);
        let ul = self.shift_up_left(1);
        let ur = self.shift_up_right(1);
        let dl = self.shift_down_left(1);
        let dr = self.shift_down_right(1);

        up.combine(down)
            .combine(left)
            .combine(right)
            .combine(ul)
            .combine(ur)
            .combine(dl)
            .combine(dr)
    }

    /// Gets all possible occupancies for a given mask
    #[inline]
    pub fn get_occupancies(&self) -> Vec<Bitboard> {
        iterate_subsets(*self).collect()
    }
}

/// Iterator that generates all subsets of a given mask
fn iterate_subsets(mask: Bitboard) -> impl Iterator<Item = Bitboard> {
    let mut subset = mask;
    let mut done = false;
    std::iter::from_fn(move || {
        if done {
            None
        } else {
            let result = subset;
            if subset.is_empty() {
                done = true;
                Some(result)
            } else {
                subset.0 = (subset.0 - 1) & mask.0;
                Some(result)
            }
        }
    })
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board = String::with_capacity(256);

        for rank in RANKS.iter().rev() {
            for file in FILES.iter() {
                let square = Square::from_file_rank(*file, *rank);
                board.push_str(if self.is_occupied(square) { "| 1 |" } else { "| 0 |" });
            }
            board.push('\n');
        }

        write!(f, "{}", board)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_up() {
        let board = Bitboard(0b0000_0000_0001);
        assert_eq!(board.shift_up(1), Bitboard(0b0001_0000_0000));
    }

    #[test]
    fn test_move_down() {
        let board = Bitboard(0b0001_0000_0000);
        assert_eq!(board.shift_down(1), Bitboard(0b0000_0000_0001));
    }

    #[test]
    fn test_move_left() {
        let board = Bitboard(0b0000_0010);
        assert_eq!(board.shift_left(1), Bitboard(0b0000_0001));
    }

    #[test]
    fn test_move_right() {
        let board = Bitboard(0b0000_0100);
        assert_eq!(board.shift_right(1), Bitboard(0b0000_1000));
    }

    #[test]
    fn test_move_up_left() {
        let board = Bitboard(0b0000_0001);
        assert_eq!(board.shift_up_left(1), Bitboard(0b0000_0000_0000));
    }

    #[test]
    fn test_move_up_right() {
        let board = Bitboard(0b0000_0001);
        assert_eq!(board.shift_up_right(1), Bitboard(0b0010_0000_0000));
    }

    #[test]
    fn test_move_down_left() {
        let board = Bitboard(0b0010_0000_0000);
        assert_eq!(board.shift_down_left(1), Bitboard(0b0000_0001));
    }

    #[test]
    fn test_move_down_right() {
        let board = Bitboard(0b0000_0100);
        assert_eq!(board.shift_down_right(1), Bitboard(0b0000_0000_0000));
    }
}
