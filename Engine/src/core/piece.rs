#[rustfmt::skip]
pub const PIECE_CHARS_FANCY: [char; 13] = [' ', '♟', '♝', '♞', '♜', '♛', '♚', '♙', '♗', '♘', '♖', '♕', '♔'];

#[rustfmt::skip]
pub const PIECE_CHARS: [char; 13] = [' ', 'P', 'B', 'N', 'R', 'Q', 'K', 'p', 'b', 'n', 'r', 'q', 'k'];

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Piece {
    EMPTY = 0,
    WP = 1,
    WB = 2,
    WN = 3,
    WR = 4,
    WQ = 5,
    WK = 6,
    BP = 7,
    BB = 8,
    BN = 9,
    BR = 10,
    BQ = 11,
    BK = 12,
}

pub const PIECES: [Piece; 13] = [
    Piece::EMPTY,
    Piece::WP,
    Piece::WB,
    Piece::WN,
    Piece::WR,
    Piece::WQ,
    Piece::WK,
    Piece::BP,
    Piece::BB,
    Piece::BN,
    Piece::BR,
    Piece::BQ,
    Piece::BK,
];

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
    Both = 2,
}
pub const COLORS: [Color; 3] = [Color::White, Color::Black, Color::Both];

impl Piece {
    pub fn from_char(c: char) -> Piece {
        match c {
            'P' => Piece::WP,
            'B' => Piece::WB,
            'N' => Piece::WN,
            'R' => Piece::WR,
            'Q' => Piece::WQ,
            'K' => Piece::WK,
            'p' => Piece::BP,
            'b' => Piece::BB,
            'n' => Piece::BN,
            'r' => Piece::BR,
            'q' => Piece::BQ,
            'k' => Piece::BK,
            _ => Piece::EMPTY,
        }
    }

    pub fn get_color(&self) -> Color {
        match *self {
            Piece::EMPTY => Color::Both,
            Piece::WP | Piece::WB | Piece::WN | Piece::WR | Piece::WQ | Piece::WK => Color::White,
            Piece::BP | Piece::BB | Piece::BN | Piece::BR | Piece::BQ | Piece::BK => Color::Black,
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == Piece::EMPTY
    }
    pub fn is_pawn(&self) -> bool {
        *self == Piece::WP || *self == Piece::BP
    }
    pub fn is_bishop(&self) -> bool {
        *self == Piece::WB || *self == Piece::BB
    }
    pub fn is_knight(&self) -> bool {
        *self == Piece::WN || *self == Piece::BN
    }
    pub fn is_rook(&self) -> bool {
        *self == Piece::WR || *self == Piece::BR
    }
    pub fn is_queen(&self) -> bool {
        *self == Piece::WQ || *self == Piece::BQ
    }
    pub fn is_king(&self) -> bool {
        *self == Piece::WK || *self == Piece::BK
    }
    pub fn is_slider(self) -> bool {
        self.is_rook() || self.is_bishop() || self.is_queen()
    }

    pub fn is_same_color(&self, other: &Piece) -> bool {
        self.get_color() == other.get_color()
    }

    pub fn to_char(&self) -> char {
        PIECE_CHARS[*self as usize]
    }

    pub fn to_fancy_char(&self) -> char {
        PIECE_CHARS_FANCY[*self as usize]
    }
}

impl<T> std::ops::Index<Piece> for [T] {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<Piece> for [T] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> std::ops::Index<Piece> for Vec<T> {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<Piece> for Vec<T> {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> std::ops::Index<Color> for [T] {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<Color> for [T] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> std::ops::Index<Color> for Vec<T> {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<Color> for Vec<T> {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_from_char() {
        assert_eq!(Piece::from_char('P'), Piece::WP);
        assert_eq!(Piece::from_char('n'), Piece::BN);
        assert_eq!(Piece::from_char('x'), Piece::EMPTY);
    }

    #[test]
    fn piece_to_char() {
        assert_eq!(Piece::WR.to_char(), 'R');
        assert_eq!(Piece::BB.to_char(), 'b');
        assert_eq!(Piece::EMPTY.to_char(), ' ');
    }

    #[test]
    fn piece_to_fancy_char() {
        assert_eq!(Piece::WN.to_fancy_char(), '♞');
        assert_eq!(Piece::BP.to_fancy_char(), '♙');
        assert_eq!(Piece::EMPTY.to_fancy_char(), ' ');
    }

    #[test]
    fn piece_get_color() {
        assert_eq!(Piece::WP.get_color(), Color::White);
        assert_eq!(Piece::BK.get_color(), Color::Black);
        assert_eq!(Piece::EMPTY.get_color(), Color::Both);
    }

    #[test]
    fn piece_is_same_color() {
        assert_eq!(Piece::WP.is_same_color(&Piece::WR), true);
        assert_eq!(Piece::WK.is_same_color(&Piece::BR), false);
    }

    #[test]
    fn piece_is_piece_type() {
        assert_eq!(Piece::WB.is_bishop(), true);
        assert_eq!(Piece::BQ.is_queen(), true);
        assert_eq!(Piece::BP.is_pawn(), true);
        assert_eq!(Piece::WQ.is_pawn(), false);
    }

    #[test]
    fn piece_is_slider() {
        assert_eq!(Piece::WK.is_slider(), false);
        assert_eq!(Piece::BR.is_slider(), true);
        assert_eq!(Piece::WQ.is_slider(), true);
    }

    #[test]
    fn index_test() {
        let vec = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'];
        assert_eq!(vec[Piece::WB], 'c');
        assert_eq!(vec[Piece::BN], 'j');
    }

    #[test]
    fn index_mut_test() {
        let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        vec[Piece::WQ] = 99;
        assert_eq!(vec[Piece::WQ], 99);
    }

    #[test]
    fn color_index_test() {
        let vec = vec!['a', 'b', 'c'];
        assert_eq!(vec[Color::White], 'a');
        assert_eq!(vec[Color::Black], 'b');
    }

    #[test]
    fn color_index_mut_test() {
        let mut vec = vec![0, 1, 2];
        vec[Color::Both] = 99;
        assert_eq!(vec[Color::Both], 99);
    }
}
