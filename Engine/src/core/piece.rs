#[rustfmt::skip]
pub const PIECE_CHARS_FANCY: [char; 13] = ['♟', '♞', '♝', '♜', '♛', '♚', '♙', '♘', '♗', '♖', '♕', '♔', ' '];
#[rustfmt::skip]
pub const PIECE_CHARS: [char; 13] = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k', ' '];

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CastlePerms {
    WKC = 1,
    WQC = 2,
    BKC = 4,
    BQC = 8,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    WPawn = 0,
    WKnight = 1,
    WBishop = 2,
    WRook = 3,
    WQueen = 4,
    WKing = 5,
    BPawn = 6,
    BKnight = 7,
    BBishop = 8,
    BRook = 9,
    BQueen = 10,
    BKing = 11,
    None = 12,
}

pub const PIECES: [Piece; 12] = [
    Piece::WPawn,
    Piece::WKnight,
    Piece::WBishop,
    Piece::WRook,
    Piece::WQueen,
    Piece::WKing,
    Piece::BPawn,
    Piece::BKnight,
    Piece::BBishop,
    Piece::BRook,
    Piece::BQueen,
    Piece::BKing,
];

pub const PIECE_TYPES: [PieceType; 6] = [
    PieceType::Pawn,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Rook,
    PieceType::Queen,
    PieceType::King,
];

impl Piece {
    pub fn new(color: Color, ptype: PieceType) -> Piece {
        match (ptype, color) {
            (PieceType::Pawn, Color::White) => Piece::WPawn,
            (PieceType::Knight, Color::White) => Piece::WKnight,
            (PieceType::Bishop, Color::White) => Piece::WBishop,
            (PieceType::Rook, Color::White) => Piece::WRook,
            (PieceType::Queen, Color::White) => Piece::WQueen,
            (PieceType::King, Color::White) => Piece::WKing,
            (PieceType::Pawn, Color::Black) => Piece::BPawn,
            (PieceType::Knight, Color::Black) => Piece::BKnight,
            (PieceType::Bishop, Color::Black) => Piece::BBishop,
            (PieceType::Rook, Color::Black) => Piece::BRook,
            (PieceType::Queen, Color::Black) => Piece::BQueen,
            (PieceType::King, Color::Black) => Piece::BKing,
        }
    }

    pub fn from_char(c: char) -> Result<Piece, &'static str> {
        match c {
            'P' => Ok(Piece::WPawn),
            'N' => Ok(Piece::WKnight),
            'B' => Ok(Piece::WBishop),
            'R' => Ok(Piece::WRook),
            'Q' => Ok(Piece::WQueen),
            'K' => Ok(Piece::WKing),
            'p' => Ok(Piece::BPawn),
            'n' => Ok(Piece::BKnight),
            'b' => Ok(Piece::BBishop),
            'r' => Ok(Piece::BRook),
            'q' => Ok(Piece::BQueen),
            'k' => Ok(Piece::BKing),
            _ => Err("Invalid piece char"),
        }
    }

    pub fn to_char(self) -> char {
        PIECE_CHARS[self as usize]
    }

    pub fn to_char_fancy(self) -> char {
        PIECE_CHARS_FANCY[self as usize]
    }

    #[inline]
    pub fn to_index(self) -> usize {
        self as usize
    }

    #[inline]
    pub fn from_index(index: usize) -> Piece {
        PIECES[index]
    }

    pub fn get_color(self) -> Color {
        match self {
            Piece::WPawn | Piece::WKnight | Piece::WBishop | Piece::WRook | Piece::WQueen | Piece::WKing => Color::White,
            Piece::BPawn | Piece::BKnight | Piece::BBishop | Piece::BRook | Piece::BQueen | Piece::BKing => Color::Black,
            Piece::None => panic!("None piece has no color"),
        }
    }

    pub fn get_type(self) -> PieceType {
        match self {
            Piece::WPawn | Piece::BPawn => PieceType::Pawn,
            Piece::WKnight | Piece::BKnight => PieceType::Knight,
            Piece::WBishop | Piece::BBishop => PieceType::Bishop,
            Piece::WRook | Piece::BRook => PieceType::Rook,
            Piece::WQueen | Piece::BQueen => PieceType::Queen,
            Piece::WKing | Piece::BKing => PieceType::King,
            Piece::None => panic!("None piece has no type"),
        }
    }

    pub fn get_value(self) -> i32 {
        match self {
            Piece::WPawn | Piece::BPawn => 100,
            Piece::WKnight | Piece::BKnight => 320,
            Piece::WBishop | Piece::BBishop => 330,
            Piece::WRook | Piece::BRook => 500,
            Piece::WQueen | Piece::BQueen => 900,
            Piece::WKing | Piece::BKing => 100000,
            Piece::None => 0,
        }
    }

    pub fn is_same_color(self, other: Piece) -> bool {
        self.get_color() == other.get_color()
    }

    pub fn is_same_type(self, other: Piece) -> bool {
        self.get_type() == other.get_type()
    }

    pub fn is_pawn(&self) -> bool {
        *self == Piece::WPawn || *self == Piece::BPawn
    }

    pub fn is_bishop(&self) -> bool {
        *self == Piece::WBishop || *self == Piece::BBishop
    }

    pub fn is_knight(&self) -> bool {
        *self == Piece::WKnight || *self == Piece::BKnight
    }

    pub fn is_rook(&self) -> bool {
        *self == Piece::WRook || *self == Piece::BRook
    }

    pub fn is_queen(&self) -> bool {
        *self == Piece::WQueen || *self == Piece::BQueen
    }

    pub fn is_king(&self) -> bool {
        *self == Piece::WKing || *self == Piece::BKing
    }

    pub fn is_slider(self) -> bool {
        self.is_rook() || self.is_bishop() || self.is_queen()
    }
}

impl PieceType {
    #[inline]
    pub fn to_index(self) -> usize {
        self as usize
    }

    #[inline]
    pub fn from_index(index: usize) -> PieceType {
        PIECE_TYPES[index]
    }

    pub fn get_value(self) -> i32 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 10000,
        }
    }
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub fn to_index(self) -> usize {
        self as usize
    }

    pub fn get_factor(self) -> i32 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
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
        assert_eq!(Piece::from_char('P').is_ok(), true);
        assert_eq!(Piece::from_char('n').is_ok(), true);
        assert_eq!(Piece::from_char('Z').is_err(), true);
    }

    #[test]
    fn piece_to_char() {
        assert_eq!(Piece::WRook.to_char(), 'R');
        assert_eq!(Piece::BBishop.to_char(), 'b');
    }

    #[test]
    fn piece_to_fancy_char() {
        assert_eq!(Piece::WKnight.to_char_fancy(), '♞');
        assert_eq!(Piece::BPawn.to_char_fancy(), '♙');
    }

    #[test]
    fn piece_get_color() {
        assert_eq!(Piece::WPawn.get_color(), Color::White);
        assert_eq!(Piece::BKing.get_color(), Color::Black);
    }

    #[test]
    fn piece_is_same_color() {
        assert_eq!(Piece::WPawn.is_same_color(Piece::WRook), true);
        assert_eq!(Piece::WKing.is_same_color(Piece::BRook), false);
    }

    #[test]
    fn piece_is_piece_type() {
        assert_eq!(Piece::WBishop.is_bishop(), true);
        assert_eq!(Piece::BQueen.is_queen(), true);
        assert_eq!(Piece::BPawn.is_pawn(), true);
        assert_eq!(Piece::WQueen.is_pawn(), false);
    }

    #[test]
    fn piece_is_slider() {
        assert_eq!(Piece::WKing.is_slider(), false);
        assert_eq!(Piece::BRook.is_slider(), true);
        assert_eq!(Piece::WQueen.is_slider(), true);
    }

    #[test]
    fn index_test() {
        let vec = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'];
        assert_eq!(vec[Piece::WBishop], 'c');
        assert_eq!(vec[Piece::BKnight], 'h');
    }

    #[test]
    fn index_mut_test() {
        let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        vec[Piece::WQueen] = 99;
        assert_eq!(vec[Piece::WQueen], 99);
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
        vec[Color::Black] = 99;
        assert_eq!(vec[Color::Black], 99);
    }
}
