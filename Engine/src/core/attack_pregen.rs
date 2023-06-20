use std::ops::{BitAnd, Shr};

use super::{
    bit_masks::*,
    bitboard::Bitboard,
    piece::Color,
    square::{Square, SQUARES},
};

const BISHOP_MAGICS: [u64; 64] = [
    0x0002020202020200u64,
    0x0002020202020000u64,
    0x0004010202000000u64,
    0x0004040080000000u64,
    0x0001104000000000u64,
    0x0000821040000000u64,
    0x0000410410400000u64,
    0x0000104104104000u64,
    0x0000040404040400u64,
    0x0000020202020200u64,
    0x0000040102020000u64,
    0x0000040400800000u64,
    0x0000011040000000u64,
    0x0000008210400000u64,
    0x0000004104104000u64,
    0x0000002082082000u64,
    0x0004000808080800u64,
    0x0002000404040400u64,
    0x0001000202020200u64,
    0x0000800802004000u64,
    0x0000800400A00000u64,
    0x0000200100884000u64,
    0x0000400082082000u64,
    0x0000200041041000u64,
    0x0002080010101000u64,
    0x0001040008080800u64,
    0x0000208004010400u64,
    0x0000404004010200u64,
    0x0000840000802000u64,
    0x0000404002011000u64,
    0x0000808001041000u64,
    0x0000404000820800u64,
    0x0001041000202000u64,
    0x0000820800101000u64,
    0x0000104400080800u64,
    0x0000020080080080u64,
    0x0000404040040100u64,
    0x0000808100020100u64,
    0x0001010100020800u64,
    0x0000808080010400u64,
    0x0000820820004000u64,
    0x0000410410002000u64,
    0x0000082088001000u64,
    0x0000002011000800u64,
    0x0000080100400400u64,
    0x0001010101000200u64,
    0x0002020202000400u64,
    0x0001010101000200u64,
    0x0000410410400000u64,
    0x0000208208200000u64,
    0x0000002084100000u64,
    0x0000000020880000u64,
    0x0000001002020000u64,
    0x0000040408020000u64,
    0x0004040404040000u64,
    0x0002020202020000u64,
    0x0000104104104000u64,
    0x0000002082082000u64,
    0x0000000020841000u64,
    0x0000000000208800u64,
    0x0000000010020200u64,
    0x0000000404080200u64,
    0x0000040404040400u64,
    0x0002020202020200u64,
];

const ROOK_MAGICS: [u64; 64] = [
    0x0080001020400080u64,
    0x0040001000200040u64,
    0x0080081000200080u64,
    0x0080040800100080u64,
    0x0080020400080080u64,
    0x0080010200040080u64,
    0x0080008001000200u64,
    0x0080002040800100u64,
    0x0000800020400080u64,
    0x0000400020005000u64,
    0x0000801000200080u64,
    0x0000800800100080u64,
    0x0000800400080080u64,
    0x0000800200040080u64,
    0x0000800100020080u64,
    0x0000800040800100u64,
    0x0000208000400080u64,
    0x0000404000201000u64,
    0x0000808010002000u64,
    0x0000808008001000u64,
    0x0000808004000800u64,
    0x0000808002000400u64,
    0x0000010100020004u64,
    0x0000020000408104u64,
    0x0000208080004000u64,
    0x0000200040005000u64,
    0x0000100080200080u64,
    0x0000080080100080u64,
    0x0000040080080080u64,
    0x0000020080040080u64,
    0x0000010080800200u64,
    0x0000800080004100u64,
    0x0000204000800080u64,
    0x0000200040401000u64,
    0x0000100080802000u64,
    0x0000080080801000u64,
    0x0000040080800800u64,
    0x0000020080800400u64,
    0x0000020001010004u64,
    0x0000800040800100u64,
    0x0000204000808000u64,
    0x0000200040008080u64,
    0x0000100020008080u64,
    0x0000080010008080u64,
    0x0000040008008080u64,
    0x0000020004008080u64,
    0x0000010002008080u64,
    0x0000004081020004u64,
    0x0000204000800080u64,
    0x0000200040008080u64,
    0x0000100020008080u64,
    0x0000080010008080u64,
    0x0000040008008080u64,
    0x0000020004008080u64,
    0x0000800100020080u64,
    0x0000800041000080u64,
    0x00FFFCDDFCED714Au64,
    0x007FFCDDFCED714Au64,
    0x003FFFCDFFD88096u64,
    0x0000040810002101u64,
    0x0001000204080011u64,
    0x0001000204000801u64,
    0x0001000082000401u64,
    0x0001FFFAABFAD1A2u64,
];

#[rustfmt::skip]
const BISHOP_RELEVANT_BITS: [u64; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    6, 5, 5, 5, 5, 5, 5, 6,
];

#[rustfmt::skip]
const ROOK_RELEVANT_BITS: [u64; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12,
];

pub struct PregenAttacks {
    pawn: [Bitboard; 128],
    knight: [Bitboard; 64],
    king: [Bitboard; 64],

    bishop_masks: [Bitboard; 64],
    bishop_indices: [usize; 64],
    bishop: Box<[Bitboard]>,

    rook_masks: [Bitboard; 64],
    rook_indices: [usize; 64],
    rook: Box<[Bitboard]>,
}

impl Default for PregenAttacks {
    fn default() -> Self {
        PregenAttacks {
            pawn: [Bitboard(0); 128],
            knight: [Bitboard(0); 64],
            king: [Bitboard(0); 64],

            bishop_masks: [Bitboard(0); 64],
            bishop_indices: [0; 64],
            bishop: vec![Bitboard(0); 5248].into_boxed_slice(),

            rook_masks: [Bitboard(0); 64],
            rook_indices: [0; 64],
            rook: vec![Bitboard(0); 198407].into_boxed_slice(),
        }
    }
}

impl PregenAttacks {
    pub fn init() -> PregenAttacks {
        let mut attacks = PregenAttacks::default();
        init_nonsliding_attacks(&mut attacks);
        init_sliding_attacks(&mut attacks);
        attacks
    }

    pub fn get_pawn_attacks(&self, color: Color, square: Square) -> Bitboard {
        self.pawn[63 * color as usize + square as usize]
    }

    pub fn get_knight_attacks(&self, square: Square) -> Bitboard {
        self.knight[square]
    }

    pub fn get_king_attacks(&self, square: Square) -> Bitboard {
        self.king[square]
    }

    pub fn get_bishop_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let rel = BISHOP_RELEVANT_BITS[square];
        let magic_index = occupancy
            .bitand(self.bishop_masks[square])
            .as_u64()
            .wrapping_mul(BISHOP_MAGICS[square])
            .shr(64 - rel) as usize;

        return self.bishop[self.bishop_indices[square] + magic_index];
    }

    pub fn get_rook_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let rel = ROOK_RELEVANT_BITS[square];
        let magic_index = occupancy
            .bitand(self.rook_masks[square])
            .as_u64()
            .wrapping_mul(ROOK_MAGICS[square])
            .shr(64 - rel) as usize;

        return self.rook[self.rook_indices[square] + magic_index];
    }
}

//initializes pawn, king and knight attacks
fn init_nonsliding_attacks(attacks: &mut PregenAttacks) {
    for s in SQUARES {
        attacks.pawn[63 * Color::White as usize + s as usize] = gen_pawn_attack(Color::White, s);
        attacks.pawn[63 * Color::Black as usize + s as usize] = gen_pawn_attack(Color::Black, s);

        attacks.knight[s] = gen_knight_attack(s);
        attacks.king[s] = gen_king_attack(s);
    }
}

fn init_sliding_attacks(attacks: &mut PregenAttacks) {
    attacks.bishop_masks = gen_bishop_masks();
    attacks.rook_masks = gen_rook_masks();

    let mut bsum = 0;
    let mut rsum = 0;

    for sq in SQUARES {
        let rel = BISHOP_RELEVANT_BITS[sq];
        let magic = BISHOP_MAGICS[sq];
        attacks.bishop_indices[sq] = bsum;
        bsum += 1 << rel;
        for occ in gen_occupancies(attacks.bishop_masks[sq]) {
            let magic_index = occ.as_u64().wrapping_mul(magic).shr(64 - rel) as usize;
            attacks.bishop[attacks.bishop_indices[sq] + magic_index] = bishop_attack_otf(sq, occ);
        }

        let rel = ROOK_RELEVANT_BITS[sq];
        let magic = ROOK_MAGICS[sq];
        attacks.rook_indices[sq] = rsum;
        rsum += 1 << rel;
        for occ in gen_occupancies(attacks.rook_masks[sq]) {
            let magic_index = occ.as_u64().wrapping_mul(magic).shr(64 - rel) as usize;
            attacks.rook[attacks.rook_indices[sq] + magic_index] = rook_attack_otf(sq, occ);
        }
    }
}

fn gen_pawn_attack(color: Color, square: Square) -> Bitboard {
    let board: Bitboard = Bitboard::from_square(square);

    if color == Color::White {
        return ((board << 7) & !FILE_H_BB) | ((board << 9) & !FILE_A_BB);
    }

    return ((board >> 7) & !FILE_A_BB) | ((board >> 9) & !FILE_H_BB);
}

fn gen_knight_attack(square: Square) -> Bitboard {
    let board: Bitboard = Bitboard::from_square(square);

    return (((board << 17) | (board >> 15)) & !FILE_A_BB)
        | (((board << 15) | (board >> 17)) & !FILE_H_BB)
        | (((board << 10) | (board >> 6)) & !(FILE_A_BB | FILE_B_BB))
        | (((board << 6) | (board >> 10)) & !(FILE_H_BB | FILE_G_BB));
}

fn gen_king_attack(square: Square) -> Bitboard {
    let board: Bitboard = Bitboard::from_square(square);
    return (((board << 7) | (board >> 9) | (board >> 1)) & (!FILE_H_BB))
        | (((board << 9) | (board >> 7) | (board << 1)) & (!FILE_A_BB))
        | ((board >> 8) | (board << 8));
}

fn gen_bishop_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];
    for s in SQUARES {
        let rank = s.get_rank() as i8;
        let file = s.get_file() as i8;
        let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

        for &(dx, dy) in &directions {
            let mut x = file as i8 + dx;
            let mut y = rank as i8 + dy;

            while x >= 1 && x <= 6 && y >= 1 && y <= 6 {
                masks[s].set_bit(Square::from(y as u8 * 8 + x as u8));

                x += dx;
                y += dy;
            }
        }
    }
    masks
}

fn gen_rook_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];
    for s in SQUARES {
        let rank = s.get_rank() as i8;
        let file = s.get_file() as i8;
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for &(dx, dy) in &directions {
            let mut x = file + dx;
            let mut y = rank + dy;

            while x >= 1 && x <= 6 && y >= 1 && y <= 6 {
                masks[s].set_bit(Square::from(y as u8 * 8 + x as u8));

                x += dx;
                y += dy;
            }
        }
    }

    masks
}

fn bishop_attack_otf(square: Square, occupancy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard(0);

    let rank = square.get_rank() as i8;
    let file = square.get_file() as i8;
    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for &(dx, dy) in &directions {
        let mut x = file + dx;
        let mut y = rank + dy;

        while x >= 0 && x <= 7 && y >= 0 && y <= 7 {
            let sq = Square::from(y as u8 * 8 + x as u8);

            if (occupancy & Bitboard::from_square(sq)) != 0 {
                attacks.set_bit(sq);
                break;
            }

            attacks.set_bit(sq);

            x += dx;
            y += dy;
        }
    }

    attacks
}

fn rook_attack_otf(square: Square, occupancy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard(0);

    let rank = square.get_rank() as i8;
    let file = square.get_file() as i8;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(dx, dy) in &directions {
        let mut x = file + dx;
        let mut y = rank + dy;

        while x >= 0 && x <= 7 && y >= 0 && y <= 7 {
            let sq = Square::from(y as u8 * 8 + x as u8);

            if (occupancy & Bitboard::from_square(sq)) != 0 {
                attacks.set_bit(sq);
                break;
            }

            attacks.set_bit(sq);

            x += dx;
            y += dy;
        }
    }

    attacks
}

fn iterate_subsets(mask: Bitboard) -> impl Iterator<Item = Bitboard> {
    let mut subset = mask;
    let mut done: bool = false;
    std::iter::from_fn(move || {
        if done {
            None
        } else {
            let result = subset;
            if subset == 0 {
                done = true;
                Some(result)
            } else {
                subset = (subset - 1) & mask;
                Some(result)
            }
        }
    })
}

fn gen_occupancies(mask: Bitboard) -> Vec<Bitboard> {
    iterate_subsets(mask).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_attacks() {
        let attacks = PregenAttacks::init();

        // Test white pawn attacks from various squares
        assert_eq!(attacks.get_pawn_attacks(Color::White, Square::A1), Bitboard(0x0000000000000200));
        assert_eq!(attacks.get_pawn_attacks(Color::White, Square::A8), Bitboard(0x0000000000000000));
        assert_eq!(attacks.get_pawn_attacks(Color::White, Square::H1), Bitboard(0x0000000000004000));
        assert_eq!(attacks.get_pawn_attacks(Color::White, Square::H8), Bitboard(0x0000000000000000));
        assert_eq!(attacks.get_pawn_attacks(Color::White, Square::D4), Bitboard(0x0000001400000000));

        // Test black pawn attacks from various squares
        assert_eq!(attacks.get_pawn_attacks(Color::Black, Square::A1), Bitboard(0x0000000000000000));
        assert_eq!(attacks.get_pawn_attacks(Color::Black, Square::A8), Bitboard(0x0002000000000000));
        assert_eq!(attacks.get_pawn_attacks(Color::Black, Square::H1), Bitboard(0x0000000000000000));
        assert_eq!(attacks.get_pawn_attacks(Color::Black, Square::H8), Bitboard(0x0040000000000000));
        assert_eq!(attacks.get_pawn_attacks(Color::Black, Square::D4), Bitboard(0x0000000000140000));
    }

    #[test]
    fn test_knight_attacks() {
        let attacks = PregenAttacks::init();

        // Test knight attacks from various squares
        assert_eq!(attacks.get_knight_attacks(Square::A1), Bitboard(0x0000000000020400));
        assert_eq!(attacks.get_knight_attacks(Square::A8), Bitboard(0x0004020000000000));
        assert_eq!(attacks.get_knight_attacks(Square::H1), Bitboard(0x0000000000402000));
        assert_eq!(attacks.get_knight_attacks(Square::H8), Bitboard(0x0020400000000000));
        assert_eq!(attacks.get_knight_attacks(Square::D4), Bitboard(0x0000142200221400));
    }

    #[test]
    fn test_king_attacks() {
        let attacks: PregenAttacks = PregenAttacks::init();

        // Test king attacks from various squares
        assert_eq!(attacks.get_king_attacks(Square::A1), Bitboard(0x0000000000000302));
        assert_eq!(attacks.get_king_attacks(Square::A8), Bitboard(0x0203000000000000));
        assert_eq!(attacks.get_king_attacks(Square::H1), Bitboard(0x000000000000c040));
        assert_eq!(attacks.get_king_attacks(Square::H8), Bitboard(0x40c0000000000000));
        assert_eq!(attacks.get_king_attacks(Square::D4), Bitboard(0x0000001c141c0000));
    }

    #[test]
    fn test_bisop_attack() {
        let attacks: PregenAttacks = PregenAttacks::init();
        let mut bb = Bitboard(0);
        bb.set_bit(Square::C3);
        bb.set_bit(Square::B6);
        bb.set_bit(Square::G1);
        bb.set_bit(Square::G7);

        assert_eq!(attacks.get_bishop_attacks(Square::A1, Bitboard(0)), Bitboard(0x8040201008040200));
        assert_eq!(attacks.get_bishop_attacks(Square::A8, Bitboard(0)), Bitboard(0x2040810204080));
        assert_eq!(attacks.get_bishop_attacks(Square::H1, Bitboard(0)), Bitboard(0x102040810204000));
        assert_eq!(attacks.get_bishop_attacks(Square::H8, Bitboard(0)), Bitboard(0x40201008040201));
        assert_eq!(attacks.get_bishop_attacks(Square::D4, Bitboard(0)), Bitboard(0x8041221400142241));
        assert_eq!(attacks.get_bishop_attacks(Square::D4, bb), Bitboard(0x40221400142040));
    }

    #[test]
    fn test_rook_attack() {
        let attacks: PregenAttacks = PregenAttacks::init();
        let mut bb = Bitboard(0);
        bb.set_bit(Square::B4);
        bb.set_bit(Square::G4);
        bb.set_bit(Square::D6);
        bb.set_bit(Square::D2);

        assert_eq!(attacks.get_rook_attacks(Square::A1, Bitboard(0)), Bitboard(0x1010101010101fe));
        assert_eq!(attacks.get_rook_attacks(Square::A8, Bitboard(0)), Bitboard(0xfe01010101010101));
        assert_eq!(attacks.get_rook_attacks(Square::H1, Bitboard(0)), Bitboard(0x808080808080807f));
        assert_eq!(attacks.get_rook_attacks(Square::H8, Bitboard(0)), Bitboard(0x7f80808080808080));
        assert_eq!(attacks.get_rook_attacks(Square::D4, Bitboard(0)), Bitboard(0x8080808f7080808));
        assert_eq!(attacks.get_rook_attacks(Square::D4, bb), Bitboard(0x80876080800));
    }
}
