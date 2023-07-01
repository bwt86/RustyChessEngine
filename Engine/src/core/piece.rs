#[rustfmt::skip]
pub const PIECE_CHARS_FANCY: [char; 13] = ['♟', '♞', '♝', '♜', '♛', '♚', '♙', '♘', '♗', '♖', '♕', '♔', ' '];
#[rustfmt::skip]
pub const PIECE_CHARS: [char; 13] = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k', ' '];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    WPawn,
    WKnight,
    WBishop,
    WRook,
    WQueen,
    WKing,
    BPawn,
    BKnight,
    BBishop,
    BRook,
    BQueen,
    BKing,
    Empty,
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Empty,
}

pub const PIECE_TYPES: [PieceType; 6] = [
    PieceType::Pawn,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Rook,
    PieceType::Queen,
    PieceType::King,
];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
    Both,
}

pub const COLORS: [Color; 2] = [Color::White, Color::Black];

impl Piece {
    pub fn from_type(ptype: PieceType, color: Color) -> Piece {
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
            _ => Piece::Empty,
        }
    }

    pub fn from_char(c: char) -> Piece {
        match c {
            'P' => Piece::WPawn,
            'N' => Piece::WKnight,
            'B' => Piece::WBishop,
            'R' => Piece::WRook,
            'Q' => Piece::WQueen,
            'K' => Piece::WKing,
            'p' => Piece::BPawn,
            'n' => Piece::BKnight,
            'b' => Piece::BBishop,
            'r' => Piece::BRook,
            'q' => Piece::BQueen,
            'k' => Piece::BKing,
            _ => Piece::Empty,
        }
    }

    pub fn to_char(self) -> char {
        PIECE_CHARS[self as usize]
    }

    pub fn to_char_fancy(self) -> char {
        PIECE_CHARS_FANCY[self as usize]
    }

    pub fn get_color(self) -> Color {
        match self {
            Piece::WPawn | Piece::WKnight | Piece::WBishop | Piece::WRook | Piece::WQueen | Piece::WKing => Color::White,
            Piece::BPawn | Piece::BKnight | Piece::BBishop | Piece::BRook | Piece::BQueen | Piece::BKing => Color::Black,
            Piece::Empty => Color::Both,
        }
    }

    pub fn get_piece_type(self) -> PieceType {
        match self {
            Piece::WPawn | Piece::BPawn => PieceType::Pawn,
            Piece::WKnight | Piece::BKnight => PieceType::Knight,
            Piece::WBishop | Piece::BBishop => PieceType::Bishop,
            Piece::WRook | Piece::BRook => PieceType::Rook,
            Piece::WQueen | Piece::BQueen => PieceType::Queen,
            Piece::WKing | Piece::BKing => PieceType::King,
            Piece::Empty => PieceType::Empty,
        }
    }

    pub fn get_value(self) -> u32 {
        match self {
            Piece::WPawn | Piece::BPawn => 1,
            Piece::WKnight | Piece::BKnight => 3,
            Piece::WBishop | Piece::BBishop => 3,
            Piece::WRook | Piece::BRook => 5,
            Piece::WQueen | Piece::BQueen => 9,
            Piece::WKing | Piece::BKing => 1000,
            Piece::Empty => 0,
        }
    }

    pub fn is_same_color(self, other: Piece) -> bool {
        self.get_color() == other.get_color()
    }

    pub fn is_empty(&self) -> bool {
        *self == Piece::Empty
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

impl Color {
    pub fn get_opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::Both => Color::Both,
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
        assert_eq!(Piece::from_char('P'), Piece::WPawn);
        assert_eq!(Piece::from_char('n'), Piece::BKnight);
        assert_eq!(Piece::from_char('-'), Piece::Empty);
    }

    #[test]
    fn piece_to_char() {
        assert_eq!(Piece::WRook.to_char(), 'R');
        assert_eq!(Piece::BBishop.to_char(), 'b');
        assert_eq!(Piece::Empty.to_char(), ' ');
    }

    #[test]
    fn piece_to_fancy_char() {
        assert_eq!(Piece::WKnight.to_char_fancy(), '♞');
        assert_eq!(Piece::BPawn.to_char_fancy(), '♙');
        assert_eq!(Piece::Empty.to_char_fancy(), ' ');
    }

    #[test]
    fn piece_get_color() {
        assert_eq!(Piece::WPawn.get_color(), Color::White);
        assert_eq!(Piece::BKing.get_color(), Color::Black);
        assert_eq!(Piece::Empty.get_color(), Color::Both);
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
        vec[Color::Both] = 99;
        assert_eq!(vec[Color::Both], 99);
    }
}
