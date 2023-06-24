use crate::core::{bitboard::*, piece::*, square::*};

//Bit mask for castle permissions
pub const WKC: u8 = 0b0001;
pub const WQC: u8 = 0b0010;
pub const BKC: u8 = 0b0100;
pub const BQC: u8 = 0b1000;

//collection of all castle perms
pub const CASTLE_PERMS: [u8; 4] = [WKC, WQC, BKC, BQC];

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
        if capture == 0 {
            None
        } else {
            Some(PIECES[capture])
        }
    }

    pub fn get_promotion(&self) -> Option<Piece> {
        let promotion = ((self.0 >> 20) & 0b1111) as usize;
        if promotion == 0 {
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
