/// Piece values for evaluation
pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 325;
pub const BISHOP_VALUE: i32 = 350;
pub const ROOK_VALUE: i32 = 550;
pub const QUEEN_VALUE: i32 = 1000;
pub const KING_VALUE: i32 = 100000;

#[rustfmt::skip]
pub const PIECE_CHARS_FANCY: [char; 13] = ['♟', '♞', '♝', '♜', '♛', '♚', '♙', '♘', '♗', '♖', '♕', '♔', ' '];
#[rustfmt::skip]
pub const PIECE_CHARS: [char; 13] = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k', ' '];

/// Represents castle permissions for both sides
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum CastlePerms {
    WKC = 1,
    WQC = 2,
    BKC = 4,
    BQC = 8,
}

/// Represents the color of a chess piece
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Color {
    White = 0,
    Black = 1,
}

/// Represents the type of a chess piece
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

/// Represents a chess piece with its color and type
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
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
    /// Creates a new piece from a color and piece type
    #[inline(always)]
    pub const fn new(color: Color, ptype: PieceType) -> Piece {
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

    /// Converts a character to a piece
    #[inline]
    pub const fn from_char(c: char) -> Result<Piece, &'static str> {
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
            ' ' => Ok(Piece::None),
            _ => Err("Invalid piece char"),
        }
    }

    /// Converts a piece to its character representation
    #[inline(always)]
    pub const fn to_char(self) -> char {
        PIECE_CHARS[self as usize]
    }

    /// Converts a piece to its fancy character representation
    #[inline(always)]
    pub const fn to_char_fancy(self) -> char {
        PIECE_CHARS_FANCY[self as usize]
    }

    #[inline(always)]
    pub const fn to_index(self) -> usize {
        self as usize
    }

    #[inline(always)]
    pub const fn from_index(index: usize) -> Piece {
        PIECES[index]
    }

    /// Returns the color of the piece
    #[inline(always)]
    pub const fn get_color(self) -> Color {
        match self {
            Piece::WPawn | Piece::WKnight | Piece::WBishop | Piece::WRook | Piece::WQueen | Piece::WKing => Color::White,
            Piece::BPawn | Piece::BKnight | Piece::BBishop | Piece::BRook | Piece::BQueen | Piece::BKing => Color::Black,
            Piece::None => panic!("WRONG"),
        }
    }

    /// Returns the type of the piece
    #[inline(always)]
    pub const fn get_type(self) -> PieceType {
        match self {
            Piece::WPawn | Piece::BPawn => PieceType::Pawn,
            Piece::WKnight | Piece::BKnight => PieceType::Knight,
            Piece::WBishop | Piece::BBishop => PieceType::Bishop,
            Piece::WRook | Piece::BRook => PieceType::Rook,
            Piece::WQueen | Piece::BQueen => PieceType::Queen,
            Piece::WKing | Piece::BKing => PieceType::King,
            Piece::None => panic!("WRONG"),
        }
    }

    /// Returns the value of the piece for evaluation
    #[inline(always)]
    pub const fn get_value(self) -> i32 {
        match self {
            Piece::WPawn | Piece::BPawn => PAWN_VALUE,
            Piece::WKnight | Piece::BKnight => KNIGHT_VALUE,
            Piece::WBishop | Piece::BBishop => BISHOP_VALUE,
            Piece::WRook | Piece::BRook => ROOK_VALUE,
            Piece::WQueen | Piece::BQueen => QUEEN_VALUE,
            Piece::WKing | Piece::BKing => KING_VALUE,
            Piece::None => panic!("WRONG"),
        }
    }

    /// Returns true if both pieces are of the same color
    #[inline(always)]
    pub const fn is_same_color(self, other: Piece) -> bool {
        ((self as u8) < 6) == ((other as u8) < 6)
    }

    /// Returns true if both pieces are of the same type
    #[inline(always)]
    pub const fn is_same_type(self, other: Piece) -> bool {
        (self as u8) % 6 == (other as u8) % 6
    }

    /// Returns true if the piece is a pawn
    #[inline(always)]
    pub const fn is_pawn(self) -> bool {
        matches!(self, Piece::WPawn | Piece::BPawn)
    }

    /// Returns true if the piece is a bishop
    #[inline(always)]
    pub const fn is_bishop(self) -> bool {
        matches!(self, Piece::WBishop | Piece::BBishop)
    }

    /// Returns true if the piece is a knight
    #[inline(always)]
    pub const fn is_knight(self) -> bool {
        matches!(self, Piece::WKnight | Piece::BKnight)
    }

    /// Returns true if the piece is a rook
    #[inline(always)]
    pub const fn is_rook(self) -> bool {
        matches!(self, Piece::WRook | Piece::BRook)
    }

    /// Returns true if the piece is a queen
    #[inline(always)]
    pub const fn is_queen(self) -> bool {
        matches!(self, Piece::WQueen | Piece::BQueen)
    }

    /// Returns true if the piece is a king
    #[inline(always)]
    pub const fn is_king(self) -> bool {
        matches!(self, Piece::WKing | Piece::BKing)
    }

    /// Returns true if the piece is a slider (rook, bishop, or queen)
    #[inline(always)]
    pub const fn is_slider(self) -> bool {
        self.is_rook() || self.is_bishop() || self.is_queen()
    }
}

impl PieceType {
    #[inline(always)]
    pub const fn to_index(self) -> usize {
        self as usize
    }

    #[inline(always)]
    pub const fn from_index(index: usize) -> PieceType {
        PIECE_TYPES[index]
    }

    /// Returns the value of the piece type for evaluation
    #[inline(always)]
    pub const fn get_value(self) -> i32 {
        match self {
            PieceType::Pawn => PAWN_VALUE,
            PieceType::Knight => KNIGHT_VALUE,
            PieceType::Bishop => BISHOP_VALUE,
            PieceType::Rook => ROOK_VALUE,
            PieceType::Queen => QUEEN_VALUE,
            PieceType::King => KING_VALUE,
        }
    }
}

impl Color {
    /// Returns the opposite color
    #[inline(always)]
    pub const fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline(always)]
    pub const fn to_index(self) -> usize {
        self as usize
    }

    /// Returns the factor for evaluation (1 for White, -1 for Black)
    #[inline(always)]
    pub const fn get_factor(self) -> i32 {
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
