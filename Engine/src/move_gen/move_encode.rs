use crate::core::{
    bitboard::*,
    board_state::{self, BoardState},
    piece::*,
    square::*,
};

//Bit mask for castle permissions
pub const WKC: u8 = 0b0001;
pub const WQC: u8 = 0b0010;
pub const BKC: u8 = 0b0100;
pub const BQC: u8 = 0b1000;

//collection of all castle perms
pub const CASTLE_PERMS: [u8; 4] = [WKC, WQC, BKC, BQC];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move(u32);

impl Move {
    pub fn new(
        from: Square,             //6
        to: Square,               //6
        piece: Piece,             //4
        capture: Option<Piece>,   //4
        promotion: Option<Piece>, //4
        double_pawn_push: bool,   //1
        en_passant: bool,         //1
        castling: bool,           //1
    ) -> Self {
        let mut move_int = 0u32;
        move_int |= from as u32;
        move_int |= (to as u32) << 6;
        move_int |= (piece as u32) << 12;
        move_int |= (capture.unwrap_or(Piece::Empty) as u32) << 16;
        move_int |= (promotion.unwrap_or(Piece::Empty) as u32) << 20;
        move_int |= (double_pawn_push as u32) << 24;
        move_int |= (en_passant as u32) << 25;
        move_int |= (castling as u32) << 26;
        Move(move_int)
    }

    pub fn move_from_algebraic(algebraic: &str, board_state: &BoardState) -> Move {
        let is_capture = algebraic.contains("x");

        let piece = match algebraic.chars().nth(0).unwrap().to_uppercase().next().unwrap() {
            'N' => Piece::from_type(PieceType::Knight, board_state.get_side()),
            'B' => Piece::from_type(PieceType::Bishop, board_state.get_side()),
            'R' => Piece::from_type(PieceType::Rook, board_state.get_side()),
            'Q' => Piece::from_type(PieceType::Queen, board_state.get_side()),
            'K' => Piece::from_type(PieceType::King, board_state.get_side()),
            _ => Piece::from_type(PieceType::Pawn, board_state.get_side()),
        };

        if is_capture {
            let capture_index = algebraic.find("x").unwrap();
            let from = Square::from_string(&algebraic[capture_index - 2..capture_index]);
            let to = Square::from_string(&algebraic[capture_index + 1..capture_index + 3]);

            for p in PIECES {
                if p.get_color() != board_state.get_side() && board_state.get_piece_bb(p).is_occupied(to) {
                    return Move::new(from, to, piece, Some(p), None, false, false, false);
                }
            }
            return Move::new(from, to, piece, None, None, false, false, false);
        } else {
            let index = algebraic.find("-").unwrap();
            let from = Square::from_string(&algebraic[index - 2..index]);
            let to = Square::from_string(&algebraic[index + 1..index + 3]);
            return Move::new(from, to, piece, None, None, false, false, false);
        }
    }

    pub fn get_from(&self) -> Square {
        SQUARES[(self.0 & 0b111111) as usize]
    }

    pub fn get_to(&self) -> Square {
        SQUARES[((self.0 >> 6) & 0b111111) as usize]
    }

    pub fn get_piece(&self) -> Piece {
        PIECES[((self.0 >> 12) & 0b1111) as usize]
    }

    pub fn get_capture(&self) -> Option<Piece> {
        let capture = ((self.0 >> 16) & 0b1111) as usize;
        if capture == 12 {
            None
        } else {
            Some(PIECES[capture])
        }
    }

    pub fn get_promotion(&self) -> Option<Piece> {
        let promotion = ((self.0 >> 20) & 0b1111) as usize;
        if promotion == 12 {
            None
        } else {
            Some(PIECES[promotion])
        }
    }

    pub fn is_check(&self) -> bool {
        ((self.0 >> 24) & 0b1) == 1
    }

    pub fn is_check_mate(&self) -> bool {
        ((self.0 >> 25) & 0b1) == 1
    }

    pub fn is_double_pawn_push(&self) -> bool {
        ((self.0 >> 26) & 0b1) == 1
    }

    pub fn is_en_passant(&self) -> bool {
        ((self.0 >> 27) & 0b1) == 1
    }

    pub fn is_castling(&self) -> bool {
        ((self.0 >> 28) & 0b1) == 1
    }

    pub fn is_capture(&self) -> bool {
        self.get_capture().is_some()
    }

    pub fn is_promotion(&self) -> bool {
        self.get_promotion().is_some()
    }

    pub fn is_quiet(&self) -> bool {
        !self.is_capture() && !self.is_promotion()
    }

    pub fn get_score(&self) -> u32 {
        let mut score = 0;

        if self.is_capture() {
            score += self.get_capture().unwrap().get_value();
        }

        if self.is_promotion() {
            score += self.get_promotion().unwrap().get_value() + 10;
        }

        if !self.is_quiet() {
            score += 10;
        }

        if self.is_double_pawn_push() {
            score += 1;
        }

        if self.is_en_passant() {
            score += 100;
        }

        if self.is_castling() {
            score += 1;
        }

        if self.is_check() {
            score += 10;
        }

        if self.is_check_mate() {
            score += 1000;
        }

        if self.get_piece().get_piece_type() != PieceType::Pawn {
            score += 10;
        }

        score
    }

    pub fn print_move(&self) {
        println!();
        for rank in RANKS.iter().rev() {
            for file in FILES.iter() {
                print!(
                    "| {} |",
                    Bitboard::new_from_square(self.get_from()).is_occupied(Square::from_file_rank(*file, *rank)) as u8
                )
            }
            print!("--{:?}--", self.get_piece());

            for file in FILES.iter() {
                print!(
                    "| {} |",
                    Bitboard::new_from_square(self.get_to()).is_occupied(Square::from_file_rank(*file, *rank)) as u8
                )
            }
            println!();
        }
        println!();
    }
}
