use crate::{
    board_info::{
        color::Color,
        square::{get_square, get_square_temp, Square},
    },
    util::{
        bit_masks::{FILE_A, FILE_B, FILE_BB, FILE_G, FILE_H, RANKS_BB},
        util::{
            bit_scan_forward, count_bits, get_line_east, get_line_north, get_line_south,
            get_line_west, pop_lsb, print_bb, set_bit,
        },
    },
};

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

const ROOK_RELEVANT_BITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

const BISHOP_RELEVANT_BITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

pub struct PregenAttacks {
    pawn: [[u64; 64]; 2],
    knight: [u64; 64],
    king: [u64; 64],

    bishop_masks: [u64; 64],
    bishop: Box<[u64]>,

    rook_masks: [u64; 64],
    rook: Box<[u64]>,
}

impl Default for PregenAttacks {
    fn default() -> Self {
        PregenAttacks {
            pawn: [[0; 64]; 2],
            knight: [0; 64],
            king: [0; 64],

            bishop_masks: [0; 64],
            bishop: vec![0u64; 32768].into_boxed_slice(),

            rook_masks: [0; 64],
            rook: vec![0u64; 262144].into_boxed_slice(),
        }
    }
}

impl PregenAttacks {
    pub fn init() -> PregenAttacks {
        let mut attacks = PregenAttacks::default();
        init_nonsliding_attacks(&mut attacks);
        init_sliding_attacks(&mut attacks);
        return attacks;
    }

    pub fn get_pawn_attacks(&self, color: Color, sqaure: Square) -> u64 {
        return self.pawn[color][sqaure];
    }

    pub fn get_knight_attacks(&self, sqaure: Square) -> u64 {
        return self.knight[sqaure];
    }

    pub fn get_king_attacks(&self, sqaure: Square) -> u64 {
        return self.king[sqaure];
    }

    pub fn get_bishop_attacks(&self, square: Square, occupancy: u64) -> u64 {
        let mut occupancy = occupancy.to_owned();
        occupancy &= self.bishop_masks[square];
        occupancy ^= BISHOP_MAGICS[square];
        occupancy >>= 64 - BISHOP_RELEVANT_BITS[square];

        return self.bishop[64 * (square as usize) + occupancy as usize];
    }

    pub fn get_rook_attacks(&self, square: Square, occupancy: u64) -> u64 {
        let mut occupancy = occupancy.to_owned();
        occupancy &= self.rook_masks[square];
        occupancy ^= ROOK_MAGICS[square];
        occupancy >>= 64 - ROOK_RELEVANT_BITS[square];

        return self.rook[64 * (square as usize) + occupancy as usize];
    }

    pub fn print_bishop_masks(&self) {
        for x in self.bishop_masks {
            print_bb(x);
        }
    }

    pub fn print_rook_masks(&self) {
        for x in self.rook_masks {
            print_bb(x);
        }
    }
}
//initializes pawn, king and knight attacks
fn init_nonsliding_attacks(attacks: &mut PregenAttacks) {
    for square in 0..63 {
        attacks.pawn[Color::White][square] =
            gen_pawn_attack(Color::White, Square::from_u8(square as u8));
        attacks.pawn[Color::Black][square] =
            gen_pawn_attack(Color::Black, Square::from_u8(square as u8));

        attacks.knight[square] = gen_knight_attack(Square::from_u8(square as u8));
        attacks.king[square] = gen_king_attack(Square::from_u8(square as u8));
    }
}

fn init_sliding_attacks(attacks: &mut PregenAttacks) {
    for square in 0..63 {
        attacks.bishop_masks[square] = gen_bishop_mask(Square::from_u8(square as u8));
        let mut occupancy: u64;
        let mut attack_mask = attacks.bishop_masks[square];
        let mut bit_count = count_bits(attack_mask);
        let mut occupancy_index: u64 = 1 << bit_count;

        let mut magic_index: usize;
        let mut index = 0;
        while index < occupancy_index {
            occupancy = set_occupancy(index, bit_count, attack_mask);
            magic_index = ((occupancy * BISHOP_MAGICS[square])
                >> (64 - BISHOP_RELEVANT_BITS[square])) as usize;
            attacks.bishop[64 * square + magic_index] =
                bishop_attack_otf(Square::from_u8(square as u8), occupancy);
            index += 1;
        }

        attacks.rook_masks[square] = gen_rook_mask(Square::from_u8(square as u8));
        attack_mask = attacks.rook_masks[square];
        bit_count = count_bits(attack_mask);
        occupancy_index = 1 << bit_count;

        index = 0;
        while index < occupancy_index {
            occupancy = set_occupancy(index, bit_count, attack_mask);
            magic_index =
                ((occupancy * ROOK_MAGICS[square]) >> (64 - ROOK_RELEVANT_BITS[square])) as usize;
            attacks.rook[64 * square + magic_index] =
                rook_attact_otf(Square::from_u8(square as u8), occupancy);
            index += 1;
        }
    }
}

fn gen_pawn_attack(color: Color, square: Square) -> u64 {
    let board: u64 = 1 << square;

    if color == Color::White {
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

fn gen_bishop_mask(square: Square) -> u64 {
    let mut mask = 0u64;

    let tr = square.get_rank() as i8;
    let tf = square.get_file() as i8;

    let mut rank = tr + 1;
    let mut file = tf + 1;
    while rank <= 6 && file <= 6 {
        set_bit(&mut mask, get_square_temp(rank, file));
        rank += 1;
        file += 1;
    }

    rank = tr - 1;
    file = tf + 1;
    while rank >= 1 && file <= 6 {
        set_bit(&mut mask, get_square_temp(rank, file));
        rank -= 1;
        file += 1;
    }

    rank = tr + 1;
    file = tf - 1;
    while rank <= 6 && file >= 1 {
        set_bit(&mut mask, get_square_temp(rank, file));
        rank += 1;
        file -= 1;
    }

    rank = tr - 1;
    file = tf - 1;
    while rank >= 1 && file >= 1 {
        set_bit(&mut mask, get_square_temp(rank, file));
        rank -= 1;
        file -= 1;
    }

    return mask;
}

fn gen_rook_mask(square: Square) -> u64 {
    return (get_line_north(square) & !RANKS_BB[7])
        | (get_line_south(square) & !RANKS_BB[0])
        | (get_line_east(square) & !FILE_BB[7])
        | (get_line_west(square) & !FILE_BB[0]);
}

fn bishop_attack_otf(square: Square, block: u64) -> u64 {
    let mut mask = 0u64;

    let tr = square.get_rank() as i8;
    let tf = square.get_file() as i8;

    let mut rank = tr + 1;
    let mut file = tf + 1;
    while rank <= 7 && file <= 7 {
        set_bit(&mut mask, get_square_temp(rank, file));
        if (1u64 << get_square_temp(rank, file)) & block == 1 {
            break;
        }

        rank += 1;
        file += 1;
    }

    rank = tr - 1;
    file = tf + 1;
    while rank >= 0 && file <= 7 {
        set_bit(&mut mask, get_square_temp(rank, file));
        if (1u64 << get_square_temp(rank, file)) & block == 1 {
            break;
        }
        rank -= 1;
        file += 1;
    }

    rank = tr + 1;
    file = tf - 1;
    while rank <= 7 && file >= 0 {
        set_bit(&mut mask, get_square_temp(rank, file));
        if (1u64 << get_square_temp(rank, file)) & block == 1 {
            break;
        }
        rank += 1;
        file -= 1;
    }

    if tr != 0 && tf != 0 {
        rank = tr - 1;
        file = tf - 1;
        while rank >= 0 && file >= 0 {
            set_bit(&mut mask, get_square_temp(rank, file));
            if (1u64 << get_square_temp(rank, file)) & block == 1 {
                break;
            }

            rank -= 1;
            file -= 1;
        }
    }

    return mask;
}

fn rook_attact_otf(square: Square, block: u64) -> u64 {
    let mut attack = 0u64;

    let tr = square.get_rank() as i8;
    let tf = square.get_file() as i8;

    let mut rank = tr + 1;
    while rank <= 7 {
        set_bit(&mut attack, get_square_temp(rank, tf));
        if (1u64 << get_square_temp(rank, tf)) & block == 1 {
            break;
        }
        rank += 1;
    }

    rank = tr - 1;
    while rank >= 0 {
        set_bit(&mut attack, get_square_temp(rank, tf));
        if (1u64 << get_square_temp(rank, tf)) & block == 1 {
            break;
        }
        rank -= 1;
    }

    let mut file = tf + 1;
    while file <= 7 {
        set_bit(&mut attack, get_square_temp(tr, file));
        if (1u64 << get_square_temp(tr, file)) & block == 1 {
            break;
        }
        file += 1;
    }

    file = tf - 1;
    while rank >= 0 {
        set_bit(&mut attack, get_square_temp(tr, file));
        if (1u64 << get_square_temp(tr, file)) & block == 1 {
            break;
        }
        rank += 1;
    }

    return attack;
}

fn set_occupancy(index: u64, num_bits: u8, mask: u64) -> u64 {
    let mut occupancy = 0u64;
    let mut mask = mask.to_owned();

    for count in 0..num_bits {
        let bit = pop_lsb(&mut mask);

        if index & (1 << count) == 1 {
            set_bit(&mut occupancy, Square::from_u8(bit_scan_forward(bit)));
        }
    }
    return occupancy;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        println!("{}", gen_rook_mask(Square::A1));
    }
}
