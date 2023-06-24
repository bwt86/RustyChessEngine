use crate::core::square::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Bitboard(u64);

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
    pub fn new_empty() -> Bitboard {
        Bitboard(0)
    }

    pub fn new_from_u64(val: u64) -> Bitboard {
        Bitboard(val)
    }

    pub fn new_from_square(square: Square) -> Bitboard {
        Bitboard(1 << square as u64)
    }

    pub fn set_square(&mut self, square: Square) {
        self.0 |= 1 << square as u64;
    }

    pub fn clear_square(&mut self, square: Square) {
        self.0 &= !(1 << square as u64);
    }

    pub fn is_occupied(&self, square: Square) -> bool {
        self.0 & (1 << square as u64) != 0
    }

    pub fn get_ls_square(&self) -> Square {
        let lsb = self.0.trailing_zeros();
        Square::from_index(lsb as usize)
    }

    pub fn pop_ls_square(&mut self) -> Square {
        let lsb = self.get_ls_square();
        self.0 &= self.0 - 1;
        lsb
    }

    pub fn get_ms_square(&self) -> Square {
        let msb = self.0.leading_zeros();
        Square::from_index(msb as usize)
    }

    pub fn pop_ms_square(&mut self) -> Square {
        let msb = self.get_ms_square();
        self.0 &= self.0 - 1;
        msb
    }

    pub fn count_squares(&self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn get_occupied_squares(&self) -> Vec<Square> {
        let mut squares = Vec::new();

        let mut bb = self.clone();

        while !bb.is_empty() {
            squares.push(bb.pop_ls_square());
        }

        squares
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn print_bb(&self) {
        println!("{}", self);
    }

    pub fn get_rank(&self, rank: Rank) -> Bitboard {
        let rank_bb = RANKS_BB[rank as usize];
        self.combine(rank_bb)
    }

    pub fn get_file(&self, file: File) -> Bitboard {
        let file_bb = FILES_BB[file as usize];
        self.combine(file_bb)
    }

    pub fn get_magic_index(&self, mask: &Bitboard, magic: &u64, shift: &u64) -> usize {
        let index = (self.0 & mask.0).wrapping_mul(*magic) >> (64 - shift);
        index as usize
    }

    pub fn combine(&self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }

    pub fn intersect(&self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }

    pub fn diff(&self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & !other.0)
    }

    pub fn invert(&self) -> Bitboard {
        Bitboard(!self.0)
    }

    pub fn make_move(&mut self, from: Square, to: Square) {
        self.clear_square(from);
        self.set_square(to);
    }

    pub fn move_up(&self, num_sq: u8) -> Bitboard {
        Bitboard(self.0 << (8 * num_sq))
    }

    pub fn move_down(&self, num_sq: u8) -> Bitboard {
        Bitboard(self.0 >> (8 * num_sq))
    }

    pub fn move_left(&self, num_sq: u8) -> Bitboard {
        Bitboard(self.0 >> num_sq)
    }

    pub fn move_right(&self, num_sq: u8) -> Bitboard {
        Bitboard(self.0 << num_sq)
    }

    pub fn move_up_left(&self, num_sq: u8) -> Bitboard {
        Bitboard(self.0 << (7 * num_sq)).intersect(!FILE_H_BB)
    }

    pub fn move_up_right(&self, num_sq: u8) -> Bitboard {
        Bitboard(self.0 << (9 * num_sq)).intersect(!FILE_A_BB)
    }

    pub fn move_down_left(&self, num_sq: u8) -> Bitboard {
        Bitboard(self.0 >> (9 * num_sq)).intersect(!FILE_H_BB)
    }

    pub fn move_down_right(&self, num_sq: u8) -> Bitboard {
        Bitboard(self.0 >> (7 * num_sq)).intersect(!FILE_A_BB)
    }

    /*
     * The gen_occupancies function generates all the possible occupancies for a given mask.
     * The function is called in the init function of the MagicBitboard struct.
     * The function is not called directly by the user.
     */
    pub fn get_occupancies(&self) -> Vec<Bitboard> {
        iterate_subsets(*self).collect()
    }
}

/*
 * iterate_subsets is an iterator that generates all the subsets of a given mask.
 * The function is called in the init function of the MagicBitboard struct.
 * The function is not called directly by the user.
 */
fn iterate_subsets(mask: Bitboard) -> impl Iterator<Item = Bitboard> {
    let mut subset = mask;
    let mut done: bool = false;
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
        let mut board = String::new();

        for rank in RANKS.iter().rev() {
            for file in FILES.iter() {
                let square = Square::from_file_rank(*file, *rank);

                if self.is_occupied(square) {
                    board.push_str("| 1 |");
                } else {
                    board.push_str("| 0 |");
                }
            }
            board.push('\n');
        }

        write!(f, "{}", board)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Bitboard;

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
        assert_eq!(board.move_up(1), Bitboard(0b0001_0000_0000));
    }

    #[test]
    fn test_move_down() {
        let board = Bitboard(0b0001_0000_0000);
        assert_eq!(board.move_down(1), Bitboard(0b0000_0000_0001));
    }

    #[test]
    fn test_move_left() {
        let board = Bitboard(0b0000_0010);
        assert_eq!(board.move_left(1), Bitboard(0b0000_0001));
    }

    #[test]
    fn test_move_right() {
        let board = Bitboard(0b0000_0100);
        assert_eq!(board.move_right(1), Bitboard(0b0000_1000));
    }

    #[test]
    fn test_move_up_left() {
        let board = Bitboard(0b0000_0001);
        assert_eq!(board.move_up_left(1), Bitboard(0b0000_0000_0000));
    }

    #[test]
    fn test_move_up_right() {
        let board = Bitboard(0b0000_0001);
        assert_eq!(board.move_up_right(1), Bitboard(0b0010_0000_0000));
    }

    #[test]
    fn test_move_down_left() {
        let board = Bitboard(0b0010_0000_0000);
        assert_eq!(board.move_down_left(1), Bitboard(0b0000_0001));
    }

    #[test]
    fn test_move_down_right() {
        let board = Bitboard(0b0000_0100);
        assert_eq!(board.move_down_right(1), Bitboard(0b0000_0000_0000));
    }
}
