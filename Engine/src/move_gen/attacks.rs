use crate::{
    board::{color::Color, square::Square},
    util::bit_masks::{FILE_A, FILE_B, FILE_G, FILE_H},
};

pub struct Attacks {
    pawn: [[u64; 64]; 2],
    knight: [u64; 64],
    king: [u64; 64],

    bishop: Box<[u64]>,
    rook: Box<[u64]>,
}

impl Default for Attacks {
    fn default() -> Self {
        Attacks {
            pawn: [[0; 64]; 2],
            knight: [0; 64],
            king: [0; 64],

            bishop: vec![0u64; 32768].into_boxed_slice(),
            rook: vec![0u64; 262144].into_boxed_slice(),
        }
    }
}

impl Attacks {
    pub fn init() -> Attacks {
        let mut attacks = Attacks::default();
        init_nonslider_attacks(&mut attacks);
        return attacks;
    }

    pub fn get_pawn_attack(&self, color: Color, sqaure: Square) -> u64 {
        return self.pawn[color][sqaure];
    }

    pub fn get_knight_attack(&self, sqaure: Square) -> u64 {
        return self.knight[sqaure];
    }

    pub fn get_king_attack(&self, sqaure: Square) -> u64 {
        return self.king[sqaure];
    }
}
//initializes pawn, king and knight attacks
fn init_nonslider_attacks(attacks: &mut Attacks) {
    for square in 0..63 {
        attacks.pawn[Color::WHITE][square] =
            gen_pawn_attack(Color::WHITE, Square::from_u8(square as u8));
        attacks.pawn[Color::BLACK][square] =
            gen_pawn_attack(Color::BLACK, Square::from_u8(square as u8));

        attacks.knight[square] = gen_knight_attack(Square::from_u8(square as u8));
        attacks.king[square] = gen_king_attack(Square::from_u8(square as u8));
    }
}

fn gen_pawn_attack(color: Color, sqaure: Square) -> u64 {
    let board: u64 = 1 << sqaure;

    if color == Color::WHITE {
        return ((board << 7) & !FILE_H) | ((board << 9) & !FILE_A);
    }

    return ((board << 7) & !FILE_H) | ((board << 9) & !FILE_A);
}

fn gen_knight_attack(square: Square) -> u64 {
    let board: u64 = 1 << square;

    return (((board << 17) | (board >> 15)) & !FILE_A)
        | (((board << 15) | (board >> 17)) & !FILE_H)
        | (((board << 10) | (board >> 6)) & !(FILE_A | FILE_B))
        | (((board << 6) | (board >> 10)) & !(FILE_H | FILE_G));
}

fn gen_king_attack(square: Square) -> u64 {
    let board: u64 = 1 << square;
    return (((board << 7) | (board >> 9) | (board >> 1)) & (!FILE_H))
        | (((board << 9) | (board >> 7) | (board << 1)) & (!FILE_A))
        | ((board >> 8) | (board << 8));
}
