use crate::core::{
    bitboard::*,
    board_state::BoardState,
    piece::{Color, Piece, PieceType, PIECES},
    piece_square_table::{Phase, PieceSquareTable},
    square::*,
};

// Bit masks and shifts for move encoding
const FROM_MASK: u32 = 0x3F; // 6 bits
const TO_MASK: u32 = 0x3F << 6; // 6 bits
const PIECE_MASK: u32 = 0xF << 12; // 4 bits
const CAPTURE_MASK: u32 = 0xF << 16; // 4 bits
const PROMO_MASK: u32 = 0xF << 20; // 4 bits

const FROM_SHIFT: u32 = 0;
const TO_SHIFT: u32 = 6;
const PIECE_SHIFT: u32 = 12;
const CAPTURE_SHIFT: u32 = 16;
const PROMO_SHIFT: u32 = 20;

const DOUBLE_PAWN_PUSH_FLAG: u32 = 1 << 24;
const EN_PASSANT_FLAG: u32 = 1 << 25;
const CASTLING_FLAG: u32 = 1 << 26;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Move(u32);

impl Move {
    #[inline(always)]
    pub fn new(
        from: Square,
        to: Square,
        piece: Piece,
        capture: Option<Piece>,
        promotion: Option<Piece>,
        double_pawn_push: bool,
        en_passant: bool,
        castling: bool,
    ) -> Self {
        let mut move_int = 0u32;
        move_int |= (from as u32) << FROM_SHIFT;
        move_int |= (to as u32) << TO_SHIFT;
        move_int |= (piece as u32) << PIECE_SHIFT;
        move_int |= (capture.unwrap_or(Piece::None) as u32) << CAPTURE_SHIFT;
        move_int |= (promotion.unwrap_or(Piece::None) as u32) << PROMO_SHIFT;
        move_int |= (double_pawn_push as u32) << 24;
        move_int |= (en_passant as u32) << 25;
        move_int |= (castling as u32) << 26;
        Move(move_int)
    }

    #[inline(always)]
    pub fn move_from_algebraic(algebraic: &str, board_state: &BoardState) -> Result<Move, &'static str> {
        if !(4..=5).contains(&algebraic.len()) {
            return Err("Invalid algebraic notation");
        }

        let friendly = board_state.get_position_bb(board_state.get_side());
        let from = Square::from_string(&algebraic[0..2])?;
        let to = Square::from_string(&algebraic[2..4])?;

        if !friendly.is_occupied(from) {
            return Err("No piece on from square");
        }

        let piece = board_state.get_piece_on_square(from).unwrap();
        let mut capture = board_state.get_piece_on_square(to);
        let mut promotion = None;
        let mut double_pawn_push = false;
        let mut en_passant = false;
        let mut castling = false;

        if algebraic.len() == 5 {
            promotion = Some(Piece::from_char(algebraic.chars().nth(4).unwrap())?);
        }

        if piece.get_type() == PieceType::Pawn {
            let from_rank = from.get_rank();
            let to_rank = to.get_rank();
            double_pawn_push = (from_rank == Rank::R2 && to_rank == Rank::R4) || (from_rank == Rank::R7 && to_rank == Rank::R5);

            if board_state.get_en_passant().is_some_and(|sq| sq == to) {
                capture = Some(Piece::new(board_state.get_side().opposite(), PieceType::Pawn));
                en_passant = true;
            }
        }

        if piece.is_king() {
            castling = match (from, to) {
                (Square::E1, Square::G1) | (Square::E1, Square::C1) => piece == Piece::WKing,
                (Square::E8, Square::G8) | (Square::E8, Square::C8) => piece == Piece::BKing,
                _ => false,
            };
        }

        Ok(Move::new(from, to, piece, capture, promotion, double_pawn_push, en_passant, castling))
    }

    #[inline(always)]
    pub const fn get_from(&self) -> Square {
        unsafe { std::mem::transmute(((self.0 & FROM_MASK) >> FROM_SHIFT) as u8) }
    }

    #[inline(always)]
    pub const fn get_to(&self) -> Square {
        unsafe { std::mem::transmute(((self.0 & TO_MASK) >> TO_SHIFT) as u8) }
    }

    #[inline(always)]
    pub const fn get_piece(&self) -> Piece {
        PIECES[((self.0 & PIECE_MASK) >> PIECE_SHIFT) as usize]
    }

    #[inline(always)]
    pub const fn get_capture(&self) -> Option<Piece> {
        let capture = ((self.0 & CAPTURE_MASK) >> CAPTURE_SHIFT) as usize;
        if capture == 12 {
            None
        } else {
            Some(PIECES[capture])
        }
    }

    #[inline(always)]
    pub const fn get_promotion(&self) -> Option<Piece> {
        let promotion = ((self.0 & PROMO_MASK) >> PROMO_SHIFT) as usize;
        if promotion == 12 {
            None
        } else {
            Some(PIECES[promotion])
        }
    }

    #[inline(always)]
    pub const fn is_double_pawn_push(&self) -> bool {
        (self.0 & DOUBLE_PAWN_PUSH_FLAG) != 0
    }

    #[inline(always)]
    pub const fn is_en_passant(&self) -> bool {
        (self.0 & EN_PASSANT_FLAG) != 0
    }

    #[inline(always)]
    pub const fn is_castling(&self) -> bool {
        (self.0 & CASTLING_FLAG) != 0
    }

    #[inline(always)]
    pub const fn is_capture(&self) -> bool {
        (self.0 & CAPTURE_MASK) >> CAPTURE_SHIFT != 12
    }

    #[inline(always)]
    pub const fn is_promotion(&self) -> bool {
        (self.0 & PROMO_MASK) >> PROMO_SHIFT != 12
    }

    #[inline(always)]
    pub const fn is_quiet(&self) -> bool {
        self.is_capture() || self.is_promotion()
    }

    #[inline(always)]
    pub fn get_score(&self) -> i32 {
        let mut score = 0;
        let piece = self.get_piece();
        let color = piece.get_color();
        let from = self.get_from();
        let to = self.get_to();
        let rank = to.get_rank() as i32;
        let file = to.get_file() as i32;

        // MVV-LVA (Most Valuable Victim - Least Valuable Attacker)
        if let Some(captured_piece) = self.get_capture() {
            // Prioritize captures by victim value (10x) and penalize by attacker value
            score += captured_piece.get_value() * 10 - piece.get_value();
        }

        // Special moves scoring
        if self.is_castling() {
            score += 500; // High priority for castling
        }

        if let Some(promotion_piece) = self.get_promotion() {
            score += promotion_piece.get_value() * 10; // Prioritize promotions
        }

        if self.is_en_passant() {
            score += 200; // Good bonus for en passant
        }

        if self.is_double_pawn_push() {
            score += 30; // Bonus for double pawn push
        }

        // Position-based scoring using piece-square tables
        let pst = PieceSquareTable::new();

        // Penalize moving from a good square
        score -= pst.get_value(Phase::Opening, piece, from);

        // Reward moving to a good square
        score += pst.get_value(Phase::Opening, piece, to);

        // Piece-specific bonuses
        match piece.get_type() {
            PieceType::Pawn => {
                // Encourage pawn advancement and structure
                if color == Color::White {
                    // Encourage pawn advancement
                    score += rank * 10;

                    // Bonus for central pawns
                    if file >= 2 && file <= 5 {
                        score += 10;
                    }
                } else {
                    // Encourage pawn advancement
                    score += (7 - rank) * 10;

                    // Bonus for central pawns
                    if file >= 2 && file <= 5 {
                        score += 10;
                    }
                }
            }
            PieceType::Knight => {
                // Encourage development and central control
                if (color == Color::White && rank > 0) || (color == Color::Black && rank < 7) {
                    score += 20;
                }

                // Strong bonus for central knights
                if file >= 2 && file <= 5 && rank >= 2 && rank <= 5 {
                    score += 30;
                }

                // Penalty for knights on the edge
                if file == 0 || file == 7 {
                    score -= 10;
                }
            }
            PieceType::Bishop => {
                // Encourage development
                if (color == Color::White && rank > 0) || (color == Color::Black && rank < 7) {
                    score += 20;
                }

                // Bonus for central bishops
                if file >= 2 && file <= 5 && rank >= 2 && rank <= 5 {
                    score += 20;
                }
            }
            PieceType::Rook => {
                // Encourage rooks on open/semi-open files
                if (color == Color::White && rank > 0) || (color == Color::Black && rank < 7) {
                    score += 15;
                }

                // Bonus for rooks on 7th rank
                if (color == Color::White && rank == 6) || (color == Color::Black && rank == 1) {
                    score += 30;
                }
            }
            PieceType::Queen => {
                // Keep queen safe in opening
                if (color == Color::White && rank > 0) || (color == Color::Black && rank < 7) {
                    score -= 10;
                }

                // Bonus for central queen in endgame
                if file >= 2 && file <= 5 && rank >= 2 && rank <= 5 {
                    score += 10;
                }

                // Penalty for early queen development
                if (color == Color::White && rank > 1) || (color == Color::Black && rank < 6) {
                    score -= 20;
                }
            }
            PieceType::King => {
                // Encourage king safety
                if (color == Color::White && rank > 0) || (color == Color::Black && rank < 7) {
                    score -= 20;
                }

                // Bonus for castled king
                if self.is_castling() {
                    score += 50;
                }
            }
            _ => {}
        }

        // Opening/development penalty
        if piece.get_type() != PieceType::Pawn {
            score -= 10;
        }

        score
    }

    #[inline(always)]
    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(5);
        s.push_str(&self.get_from().to_string());
        s.push_str(&self.get_to().to_string());
        if let Some(p) = self.get_promotion() {
            s.push(p.to_char());
        }
        s
    }

    #[inline(always)]
    pub fn print_move(&self) {
        println!(
            "from:{}\nto: {}\npiece: {}\ncap: {}\npromo: {}\ndouble: {}\ncastling: {}\nenpassant: {}\nscore: {}",
            self.get_from().to_string(),
            self.get_to().to_string(),
            self.get_piece().to_char(),
            self.is_capture(),
            self.get_promotion().map_or(' ', |piece| piece.to_char()),
            self.is_double_pawn_push(),
            self.is_castling(),
            self.is_en_passant(),
            self.get_score(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_creation() {
        let m = Move::new(Square::E2, Square::E4, Piece::WPawn, None, None, true, false, false);
        assert_eq!(m.get_from(), Square::E2);
        assert_eq!(m.get_to(), Square::E4);
        assert_eq!(m.get_piece(), Piece::WPawn);
        assert_eq!(m.is_double_pawn_push(), true);
    }

    #[test]
    fn test_move_flags() {
        let m = Move::new(Square::E1, Square::G1, Piece::WKing, None, None, false, false, true);
        assert_eq!(m.is_castling(), true);
        assert_eq!(m.is_quiet(), true);
    }

    #[test]
    fn test_move_capture() {
        let m = Move::new(Square::E4, Square::D5, Piece::WPawn, Some(Piece::BPawn), None, false, true, false);
        assert_eq!(m.is_capture(), true);
        assert_eq!(m.is_en_passant(), true);
        assert_eq!(m.get_capture(), Some(Piece::BPawn));
    }

    #[test]
    fn test_move_promotion() {
        let m = Move::new(Square::E7, Square::E8, Piece::WPawn, None, Some(Piece::WQueen), false, false, false);
        assert_eq!(m.is_promotion(), true);
        assert_eq!(m.get_promotion(), Some(Piece::WQueen));
    }
}
