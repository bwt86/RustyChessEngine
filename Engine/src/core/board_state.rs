use crate::core::bitboard::*;
use crate::core::piece::*;
use crate::core::square::*;
use crate::move_gen::move_encode::Move;
use crate::move_gen::move_encode::BKC;
use crate::move_gen::move_encode::BQC;
use crate::move_gen::move_encode::WKC;
use crate::move_gen::move_encode::WQC;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoardState {
    piece_bb: [Bitboard; 12],
    position_bb: [Bitboard; 3],

    side: Color,
    enpas: Option<Square>,
    cast_perm: u8,
    half_move: u8,
    full_move: u32,
}

impl BoardState {
    pub fn new(fen_str: Option<&str>) -> BoardState {
        let fen = fen_str.unwrap_or("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();

        let bb_tuple = parse_piece_placment(fen_parts[0]);
        return BoardState {
            piece_bb: bb_tuple.0,
            position_bb: bb_tuple.1,

            side: parse_side(fen_parts[1]),
            cast_perm: parse_castle(fen_parts[2]),
            enpas: parse_enpas(fen_parts[3]),
            half_move: fen_parts[4].parse::<u8>().unwrap(),
            full_move: fen_parts[5].parse::<u32>().unwrap(),
        };
    }

    pub fn get_piece_bb(&self, piece: Piece) -> &Bitboard {
        &self.piece_bb[piece]
    }

    pub fn get_piecebb_from_type(&self, piece_type: PieceType, color: Color) -> &Bitboard {
        &self.piece_bb[Piece::from_type(piece_type, color)]
    }

    pub fn get_position_bb(&self, color: Color) -> &Bitboard {
        &self.position_bb[color]
    }

    pub fn get_piece_from_square(&self, square: Square) -> Option<Piece> {
        for piece in PIECES {
            if self.piece_bb[piece].is_occupied(square) {
                return Some(piece);
            }
        }
        None
    }

    pub fn get_enpas(&self) -> Option<&Square> {
        self.enpas.as_ref()
    }

    pub fn get_side(&self) -> Color {
        self.side
    }

    pub fn get_cast_perm(&self) -> u8 {
        self.cast_perm
    }

    pub fn get_half_move(&self) -> u8 {
        self.half_move
    }

    pub fn get_full_move(&self) -> u32 {
        self.full_move
    }

    pub fn make_move(&mut self, m: Move) {
        let mut side = self.side;
        let mut cast_perm = self.cast_perm;
        let mut enpas = self.enpas;
        let mut half_move = self.half_move;
        let mut full_move = self.full_move;

        let from = m.get_from();
        let to = m.get_to();
        let piece = m.get_piece();
        let capture = m.get_capture();
        let promo = m.get_promotion();
        let cast_bool = m.is_castling();

        //Update piece bitboards
        self.piece_bb[piece].make_move(from, to);

        //Update position bitboards
        self.position_bb[side].make_move(from, to);
        self.position_bb[Color::Both].make_move(from, to);

        //Update Captured piece
        if capture.is_some() {
            self.piece_bb[capture.unwrap()].clear_square(to);
            self.position_bb[side.get_opposite()].clear_square(to);
        }

        //Update promotion
        if promo.is_some() {
            self.piece_bb[piece].clear_square(to);
            self.piece_bb[promo.unwrap()].set_square(to);
        }

        //Make Castle move
        if cast_bool {
            if to == Square::G1 {
                self.piece_bb[Piece::WRook].make_move(Square::H1, Square::F1);
                self.position_bb[Color::White].make_move(Square::H1, Square::F1);
                self.position_bb[Color::Both].make_move(Square::H1, Square::F1);
            } else if to == Square::C1 {
                self.piece_bb[Piece::WRook].make_move(Square::A1, Square::D1);
                self.position_bb[Color::White].make_move(Square::A1, Square::D1);
                self.position_bb[Color::Both].make_move(Square::A1, Square::D1);
            } else if to == Square::G8 {
                self.piece_bb[Piece::BRook].make_move(Square::H8, Square::F8);
                self.position_bb[Color::Black].make_move(Square::H8, Square::F8);
                self.position_bb[Color::Both].make_move(Square::H8, Square::F8);
            } else if to == Square::C8 {
                self.piece_bb[Piece::BRook].make_move(Square::A8, Square::D8);
                self.position_bb[Color::Black].make_move(Square::A8, Square::D8);
                self.position_bb[Color::Both].make_move(Square::A8, Square::D8);
            }
        }

        //Update castling
        if piece == Piece::WRook && from == Square::A1 {
            cast_perm &= !WQC;
        } else if piece == Piece::WRook && from == Square::H1 {
            cast_perm &= !WKC;
        } else if piece == Piece::BRook && from == Square::A8 {
            cast_perm &= !BQC;
        } else if piece == Piece::BRook && from == Square::H8 {
            cast_perm &= !BKC;
        }

        if piece == Piece::WKing {
            cast_perm &= !(WKC | WQC);
        } else if piece == Piece::BKing {
            cast_perm &= !(BKC | BQC);
        }

        //Enpas move
        if enpas.is_some() {
            self.piece_bb[side.get_opposite()].clear_square(enpas.unwrap());
            self.position_bb[side.get_opposite()].clear_square(enpas.unwrap());
        }

        //Update enpas
        if enpas.is_some() {
            enpas = None;
        } else {
            if piece == Piece::WPawn && from.get_rank() == Rank::R1 && to.get_rank() == Rank::R3 {
                enpas = Some(from.move_up(1));
            } else if piece == Piece::BPawn && from.get_rank() == Rank::R8 && to.get_rank() == Rank::R6 {
                enpas = Some(to.move_down(1));
            }
        }

        //Update half move
        if promo.is_some() || capture.is_some() || piece == Piece::WPawn || piece == Piece::BPawn {
            half_move = 0;
        } else {
            half_move += 1;
        }

        //Update side
        side = side.get_opposite();

        //Update full move
        if side == Color::White {
            full_move += 1;
        }

        self.side = side;
        self.cast_perm = cast_perm;
        self.enpas = enpas;
        self.half_move = half_move;
        self.full_move = full_move;
    }

    pub fn print_board(&self) {
        println!("   A    B    C    D    E    F    G    H");

        for rank in RANKS.iter().rev() {
            print!("{}", (*rank as u8) + 1);

            for file in FILES.iter() {
                let sqaure = Square::from_file_rank(*file, *rank);
                let mut piece: char = '-';

                for p in PIECES {
                    if self.piece_bb[p].is_occupied(sqaure) {
                        piece = p.to_char_fancy();
                        break;
                    }
                }

                print!("| {} |", piece);
            }
            println!();
        }
        println!();
    }

    pub fn evaluate_position(&self, side: Color) -> i32 {
        let mut score: i32 = 0;

        for p in PIECES {
            let pcount = self.piece_bb[p].count_squares() as i32;

            if p.get_color() == side {
                score += pcount * p.get_value() as i32;
            } else {
                score -= pcount * p.get_value() as i32;
            }
        }

        score
    }

    pub fn display_info(&self) {
        println!("--------------------");
        println!("Side: {:?}", self.side);
        println!("Enpas: {:?}", self.enpas);
        println!("Cast Perm: {}", self.cast_perm);
        println!("Fifty Move: {}", self.half_move);
        println!("Full Moves: {}", self.full_move);
        println!("Eval: {}", self.evaluate_position(self.side));
        println!("--------------------");
        self.print_board();
    }

    pub fn print_all_bb(&self) {
        for p in PIECES {
            print!("Piece: {:?}", p);
            self.piece_bb[p].print_bb();
        }

        for bb in self.position_bb {
            bb.print_bb();
        }
    }
}

//Parses piece placement data from FEN
fn parse_piece_placment(fen: &str) -> ([Bitboard; 12], [Bitboard; 3]) {
    let mut file = File::FA;
    let mut rank = Rank::R8;

    let mut piece_bb: [Bitboard; 12] = [Bitboard::new_empty(); 12];
    let mut position_bb: [Bitboard; 3] = [Bitboard::new_empty(); 3];

    for piece_char in fen.chars() {
        match piece_char {
            'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                let piece = Piece::from_char(piece_char);
                init_square(
                    &mut piece_bb,
                    &mut position_bb,
                    Square::from_file_rank(file, rank),
                    piece,
                    piece.get_color(),
                );
            }
            '1' => (),
            '2' => file = file.get_next_n(1),
            '3' => file = file.get_next_n(2),
            '4' => file = file.get_next_n(3),
            '5' => file = file.get_next_n(4),
            '6' => file = file.get_next_n(5),
            '7' => file = file.get_next_n(6),
            '8' => file = file.get_next_n(7),
            '/' => continue,
            _ => panic!("Invalid FEN string"),
        }

        if file == File::FH && rank != Rank::R1 {
            rank = rank.get_prev();
        }
        file = file.get_next();
    }
    (piece_bb, position_bb)
}

//Helper funtion for parse_pieces.
//Sets bits in all relevant bit boards for each piece.
fn init_square(piece_bb: &mut [Bitboard; 12], position_bb: &mut [Bitboard; 3], sq: Square, piece: Piece, color: Color) {
    piece_bb[piece].set_square(sq);
    position_bb[color].set_square(sq);
    position_bb[Color::Both].set_square(sq);
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
            'K' => cast_perm |= WKC,
            'Q' => cast_perm |= WQC,
            'k' => cast_perm |= BKC,
            'q' => cast_perm |= BQC,
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
    Some(Square::from_string(fen_enpas).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamestate_init() {
        let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let gs = BoardState::new(None);
        assert_eq!(gs.full_move, 1);
        assert_eq!(gs.half_move, 0);
        assert_eq!(gs.side, Color::White);
        assert_eq!(gs.enpas, None);
        assert_eq!(gs.cast_perm, WKC | WQC | BKC | BQC);

        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kq - 0 2";
        let gs = BoardState::new(Some(fen));
        assert_eq!(gs.full_move, 2);
        assert_eq!(gs.half_move, 0);
        assert_eq!(gs.side, Color::White);
        assert_eq!(gs.enpas, None);
        assert_eq!(gs.cast_perm, WKC | BQC);
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
        assert_eq!(parse_castle("KQ"), WKC | WQC);
        assert_eq!(parse_castle("k"), BKC);
    }

    #[test]
    fn test_parse_side() {
        assert_eq!(parse_side("w"), Color::White);
        assert_eq!(parse_side("b"), Color::Black);
    }
}
