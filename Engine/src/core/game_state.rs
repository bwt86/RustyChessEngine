use crate::core::{
    piece::{PIECE_CHARS, PIECE_CHARS_FANCY},
    square::{FILES, RANKS},
};

use super::{
    bit_masks,
    bitboard::Bitboard,
    piece::{Color, Piece},
    square::Square,
};

struct GameState {
    piece_bb: [Bitboard; 13],
    position_bb: [Bitboard; 3],

    side: Color,
    enpas: Option<Square>,
    cast_perm: u8,
    half_move: u8,
    full_move: u32,
}

impl GameState {
    pub fn init_from_fen(fen: Option<&str>) -> GameState {
        let fen_str = fen.unwrap_or("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        build_board(fen_str)
    }

    //Moves a piece on the board and updates all relevant bit boards
    //Takes in the start square, end square, the piece that was moved, and an Optional piece that was taken.
    pub fn move_piece(
        &mut self,
        old_square: Square,
        new_square: Square,
        piece_moved: Piece,
        piece_taken: Option<Piece>,
    ) {
        self.update_piece_bb(old_square, new_square, piece_moved, piece_taken);
        self.update_postion_bb(old_square, new_square, piece_moved, piece_taken);

        self.half_move += 1;
        if piece_moved.get_color() == Color::Black {
            self.full_move += 1;
        }

        if Option::is_some(&piece_taken) || piece_moved == Piece::WP || piece_moved == Piece::WB {
            self.half_move = 0;
        }
    }

    //helper function for move_piece
    //updates all relevant piece bit boards
    fn update_piece_bb(
        &mut self,
        old_square: Square,
        new_square: Square,
        piece_moved: Piece,
        piece_taken: Option<Piece>,
    ) {
        self.piece_bb[piece_moved].make_move(old_square, new_square);

        if Option::is_some(&piece_taken) {
            self.piece_bb[piece_taken.unwrap()].make_move(old_square, new_square);
        }
    }

    //helper function for move_piece
    //updates all relevant postition bit boards
    fn update_postion_bb(
        &mut self,
        old_square: Square,
        new_square: Square,
        piece_moved: Piece,
        piece_taken: Option<Piece>,
    ) {
        self.position_bb[Color::Both].make_move(old_square, new_square);
        self.position_bb[piece_moved.get_color()].make_move(old_square, new_square);

        if Option::is_some(&piece_taken) {
            self.position_bb[Color::Both].make_move(old_square, new_square);
            self.position_bb[piece_taken.unwrap().get_color()].make_move(old_square, new_square);
        }
    }

    //Prints formant board possition
    pub fn print_board(&self) {
        println!("   A    B    C    D    E    F    G    H");

        for rank in RANKS.iter().rev() {
            print!("{}", (*rank as u8) + 1);

            for file in FILES {
                let sqaure = Square::from_rank_file(*rank, file);
                let mut piece: char = '-';

                for p in 0..12 {
                    if self.piece_bb[p].is_occupied(sqaure) {
                        piece = PIECE_CHARS_FANCY[p];
                        break;
                    }
                }

                print!("| {} |", piece);
            }
            println!();
        }
        println!();
    }

    //Displays info about the board state.
    //Prints current board possition.
    pub fn display_info(&self) {
        println!("--------------------");
        println!("Side: {:?}", self.side);
        println!("Enpas: {:?}", self.enpas);
        println!("Cast Perm: {}", self.cast_perm);
        println!("Fifty Move: {}", self.half_move);
        println!("Full Moves: {}", self.full_move);
        println!("--------------------");
        self.print_board();
    }

    pub fn print_all_bb(&self) {
        let mut x: usize = 0;
        for bb in self.piece_bb {
            print!("Piece: {}", PIECE_CHARS[x]);
            bb.print_bb();
            x += 1;
        }

        for bb in self.position_bb {
            bb.print_bb();
        }
    }
}

//Builds the board state from FEN string.
//Parses all fen parts an inits them to the board state.
//Returns a Board.
fn build_board(fen: &str) -> GameState {
    let fen_parts: Vec<&str> = fen.split_whitespace().collect();
    let bb_tuple = parse_piece_placment(fen_parts[0]);
    return GameState {
        piece_bb: bb_tuple.0,
        position_bb: bb_tuple.1,
        side: parse_side(fen_parts[1]),
        cast_perm: parse_castle(fen_parts[2]),
        enpas: parse_enpas(fen_parts[3]),
        half_move: fen_parts[4].parse::<u8>().unwrap(),
        full_move: fen_parts[5].parse::<u32>().unwrap(),
    };
}

//Parses piece placement data from FEN
fn parse_piece_placment(fen_pieces: &str) -> ([Bitboard; 13], [Bitboard; 3]) {
    let mut file: usize = 0;
    let mut rank: usize = 7;

    let mut piece_bb: [Bitboard; 13] = [Bitboard::init(0); 13];
    let mut position_bb: [Bitboard; 3] = [Bitboard::init(0); 3];

    for fen_char in fen_pieces.chars() {
        match fen_char {
            'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                let piece = Piece::from_char(fen_char);
                init_square(
                    &mut piece_bb,
                    &mut position_bb,
                    Square::from_rank_file(RANKS[rank], FILES[file]),
                    piece,
                    piece.get_color(),
                );
                file += 1;
            }
            '1' => file += 1,
            '2' => file += 2,
            '3' => file += 3,
            '4' => file += 4,
            '5' => file += 5,
            '6' => file += 6,
            '7' => file += 7,
            '8' => file += 8,
            '/' => continue,
            _ => panic!("Invalid FEN string"),
        }

        if file == 8 && rank > 0 {
            rank -= 1;
        }

        file %= 8;
    }
    (piece_bb, position_bb)
}

//Helper funtion for parse_pieces.
//Sets bits in all relevant bit boards for each piece.
fn init_square(
    piece_bb: &mut [Bitboard; 13],
    position_bb: &mut [Bitboard; 3],
    sq: Square,
    piece: Piece,
    color: Color,
) {
    piece_bb[piece].set_bit(sq);
    position_bb[color].set_bit(sq);
    position_bb[Color::Both].set_bit(sq);
}

//parses side fen string and sets state
fn parse_side(fen_side: &str) -> Color {
    if fen_side.eq("b") {
        return Color::Black;
    }
    return Color::White;
}

//parses castle permistion fen string and sets state
fn parse_castle(fen_castle: &str) -> u8 {
    let mut cast_perm: u8 = 0;
    for fen_char in fen_castle.chars() {
        match fen_char {
            'K' => cast_perm |= bit_masks::WKC,
            'Q' => cast_perm |= bit_masks::WQC,
            'k' => cast_perm |= bit_masks::BKC,
            'q' => cast_perm |= bit_masks::BQC,
            _ => continue,
        }
    }
    cast_perm
}

//parses enpas fen string and sets state
fn parse_enpas(fen_enpas: &str) -> Option<Square> {
    if fen_enpas.len() == 1 {
        return None;
    }
    let c = fen_enpas.as_bytes();

    let file: usize = ((c[0] - 97) % 9).into();
    let rank: usize = (c[1] - 49).into();

    Some(Square::from_rank_file(RANKS[rank], FILES[file]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamestate_init() {
        let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let gs = GameState::init_from_fen(None);
        assert_eq!(gs.full_move, 1);
        assert_eq!(gs.half_move, 0);
        assert_eq!(gs.side, Color::White);
        assert_eq!(gs.enpas, None);
        assert_eq!(
            gs.cast_perm,
            bit_masks::WKC | bit_masks::WQC | bit_masks::BKC | bit_masks::BQC
        );

        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kq - 0 2";
        let gs = GameState::init_from_fen(Some(fen));
        assert_eq!(gs.full_move, 2);
        assert_eq!(gs.half_move, 0);
        assert_eq!(gs.side, Color::White);
        assert_eq!(gs.enpas, None);
        assert_eq!(gs.cast_perm, bit_masks::WKC | bit_masks::BQC);
    }

    #[test]
    fn test_parse_enpas() {
        assert_eq!(parse_enpas("-"), None);
        assert_eq!(parse_enpas("e3"), Some(Square::E3));
        assert_eq!(parse_enpas("h6"), Some(Square::H6));
    }

    #[test]
    fn test_parse_castle() {
        assert_eq!(parse_castle("-"), 0);
        assert_eq!(parse_castle("KQ"), bit_masks::WKC | bit_masks::WQC);
        assert_eq!(parse_castle("k"), bit_masks::BKC);
    }

    #[test]
    fn test_parse_side() {
        assert_eq!(parse_side("w"), Color::White);
        assert_eq!(parse_side("b"), Color::Black);
    }
}
