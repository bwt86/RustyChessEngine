//Bit board of each file
pub const FILE_A: u64 = 0x101010101010101u64;
pub const FILE_B: u64 = 0x202020202020202u64;
pub const FILE_C: u64 = 0x404040404040404u64;
pub const FILE_D: u64 = 0x808080808080808u64;
pub const FILE_E: u64 = 0x1010101010101010u64;
pub const FILE_F: u64 = 0x2020202020202020u64;
pub const FILE_G: u64 = 0x4040404040404040u64;
pub const FILE_H: u64 = 0x8080808080808080u64;
//Collection of all files
pub const FILE_BB: [u64; 8] = [FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H];

//Bit board of each rank
pub const RANK_1: u64 = 0xffu64;
pub const RANK_2: u64 = 0xff00u64;
pub const RANK_3: u64 = 0xff0000u64;
pub const RANK_4: u64 = 0xff000000u64;
pub const RANK_5: u64 = 0xff00000000u64;
pub const RANK_6: u64 = 0xff0000000000u64;
pub const RANK_7: u64 = 0xff000000000000u64;
pub const RANK_8: u64 = 0xff00000000000000u64;
//Collection of all ranks
pub const RANKS_BB: [u64; 8] = [RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8];

//Bit board of all dark squares
pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;

//Bit board of all light squares
pub const LIGHT_SQUARES: u64 = !DARK_SQUARES;

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
