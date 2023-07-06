use crate::core::{
    bitboard::*,
    board_state::{self, BoardState},
    piece::{self, *},
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

    pub fn move_from_algebraic(algebraic: &str, board_state: &BoardState) -> Result<Move, &'static str> {
        if algebraic.len() < 4 || algebraic.len() > 5 {
            return Err("Invalid algebraic notation");
        }

        let friendly = board_state.get_position_bb(board_state.get_side());
        let enemy = board_state.get_position_bb(board_state.get_side().get_opposite());
        let from = Square::from_string(&algebraic[0..2])?;
        let to = Square::from_string(&algebraic[2..4])?;

        if !friendly.is_occupied(from) {
            return Err("No piece on from square");
        }

        let piece = board_state.get_piece_from_square(from).unwrap();
        let mut capture = None;
        let mut promotion = None;
        let mut double_pawn_push = false;
        let mut en_passant = false;
        let mut castling = false;

        if enemy.is_occupied(to) {
            capture = board_state.get_piece_from_square(to);
        }

        if algebraic.len() == 5 {
            promotion = Some(Piece::from_char(algebraic.chars().nth(4).unwrap()));
        }

        if piece.get_piece_type() == PieceType::Pawn
            && ((from.get_rank() == Rank::R2 && to.get_rank() == Rank::R4) || (from.get_rank() == Rank::R7 && to.get_rank() == Rank::R5))
        {
            double_pawn_push = true;
        }

        if piece.get_piece_type() == PieceType::Pawn && board_state.get_enpas().is_some_and(|sq| *sq == to) {
            capture = Some(Piece::from_type(PieceType::Pawn, board_state.get_side().get_opposite()));
            en_passant = true;
        }

        if piece == Piece::WKing && ((from == Square::E1 && to == Square::G1) || (from == Square::E1 && to == Square::C1)) {
            castling = true;
        }

        if piece == Piece::BKing && ((from == Square::E8 && to == Square::G8) || (from == Square::E8 && to == Square::C8)) {
            castling = true;
        }

        return Ok(Move::new(from, to, piece, capture, promotion, double_pawn_push, en_passant, castling));
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

    pub fn is_double_pawn_push(&self) -> bool {
        ((self.0 >> 24) & 0b1) == 1
    }

    pub fn is_en_passant(&self) -> bool {
        ((self.0 >> 25) & 0b1) == 1
    }

    pub fn is_castling(&self) -> bool {
        ((self.0 >> 26) & 0b1) == 1
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

        if self.get_piece().get_piece_type() != PieceType::Pawn {
            score += 10;
        }

        score
    }

    pub fn print_move(&self) {
        println!(
            "from:{}\nto: {}\npiece: {}\ncap: {}\npromo: {}\ndouble: {}\ncastling: {}\nenpassant: {}",
            self.get_from().to_string(),
            self.get_to().to_string(),
            self.get_piece().to_char(),
            self.is_capture(),
            self.get_promotion().map_or(' ', |piece| piece.to_char()),
            self.is_double_pawn_push(),
            self.is_castling(),
            self.is_en_passant(),
        );
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
