use super::bitboard::Bitboard;

//Bit board of each file
pub const FILE_A_BB: Bitboard = Bitboard(0x0101010101010101);
pub const FILE_B_BB: Bitboard = Bitboard(0x0202020202020202);
pub const FILE_C_BB: Bitboard = Bitboard(0x0404040404040404);
pub const FILE_D_BB: Bitboard = Bitboard(0x0808080808080808);
pub const FILE_E_BB: Bitboard = Bitboard(0x1010101010101010);
pub const FILE_F_BB: Bitboard = Bitboard(0x2020202020202020);
pub const FILE_G_BB: Bitboard = Bitboard(0x4040404040404040);
pub const FILE_H_BB: Bitboard = Bitboard(0x8080808080808080);

pub const FILES_BB: [Bitboard; 8] = [FILE_A_BB, FILE_B_BB, FILE_C_BB, FILE_D_BB, FILE_E_BB, FILE_F_BB, FILE_G_BB, FILE_H_BB];

//Bit board of each rank
pub const RANK_1_BB: Bitboard = Bitboard(0x00000000000000ff);
pub const RANK_2_BB: Bitboard = Bitboard(0x000000000000ff00);
pub const RANK_3_BB: Bitboard = Bitboard(0x0000000000ff0000);
pub const RANK_4_BB: Bitboard = Bitboard(0x00000000ff000000);
pub const RANK_5_BB: Bitboard = Bitboard(0x000000ff00000000);
pub const RANK_6_BB: Bitboard = Bitboard(0x0000ff0000000000);
pub const RANK_7_BB: Bitboard = Bitboard(0x00ff000000000000);
pub const RANK_8_BB: Bitboard = Bitboard(0xff00000000000000);

pub const RANKS_BB: [Bitboard; 8] = [RANK_1_BB, RANK_2_BB, RANK_3_BB, RANK_4_BB, RANK_5_BB, RANK_6_BB, RANK_7_BB, RANK_8_BB];

pub const DARK_SQUARE_BB: Bitboard = Bitboard(0xAA55AA55AA55AA55);
pub const LIGHT_SQUARE_BB: Bitboard = Bitboard(0xAA55AA55AA55AA55);

//Bit mask for castle permissions
pub const WKC: u8 = 0b0001;
pub const WQC: u8 = 0b0010;
pub const BKC: u8 = 0b0100;
pub const BQC: u8 = 0b1000;

//collection of all castle perms
pub const CASTLE_PERMS: [u8; 4] = [WKC, WQC, BKC, BQC];

//Amount to shift a bit to move in a given direction on a bitboard
pub const NORTH: i8 = 8;
pub const SOUTH: i8 = -8;
pub const WEST: i8 = -1;
pub const EAST: i8 = 1;

pub const NORTH_EAST: i8 = 9;
pub const NORTH_WEST: i8 = 7;
pub const SOUTH_EAST: i8 = -7;
pub const SOUTH_WEST: i8 = -9;
