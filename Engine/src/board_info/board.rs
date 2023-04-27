use crate::{
    board_info::{
        piece::{PIECE_CHARS, PIECE_CHARS_FANCY},
        square::get_square,
    },
    util::{
        bit_masks::{BKC, BQC, WKC, WQC},
        util::{is_occupied, print_bb},
    },
};

use super::{color::Color, piece::Piece, square::Square};

pub struct Board {
    piece_bb: [u64; 12],
    position_bb: [u64; 3],

    side: Color,
    enpas: Option<Square>,
    cast_perm: u8,
    half_move: u8,
    full_move: u32,
}

pub struct Move {
    piece_moved: Piece,
    piece_taken: Option<Piece>,
    old_square: Square,
    new_square: Square,
    prev_board_state: Board,
}

impl Board {
    //Inits the board state with Optional FEN string.
    //If None then init with default starting board
    pub fn init(fen_option: Option<&str>) -> Board {
        match fen_option {
            Some(fen) => return build_board(fen),
            None => return build_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        }
    }

    //Moves a piece on the board and updates all relevant bit boards
    //Takes in the start square, end square, the piece that was moved, and an Optional piece that was taken.
    pub fn move_piece(&mut self, old_square: Square, new_square: Square, piece_moved: Piece, piece_taken: Option<Piece>) {
        self.update_piece_bb(old_square, new_square, piece_moved, piece_taken);
        self.update_postion_bb(old_square, new_square, piece_moved, piece_taken);

        self.half_move += 1;
        if piece_moved.get_color() == Color::Black {
            self.full_move += 1;
        }

        if Option::is_some(&piece_taken) || piece_moved == Piece::WP || piece_moved == Piece::WB {
            self.half_move = 0;
        }
    }

    //helper function for move_piece
    //updates all relevant piece bit boards
    fn update_piece_bb(&mut self, old_square: Square, new_square: Square, piece_moved: Piece, piece_taken: Option<Piece>) {
        self.piece_bb[piece_moved] |= 1 << new_square;
        self.piece_bb[piece_moved] ^= 1 << old_square;

        if Option::is_some(&piece_taken) {
            self.piece_bb[piece_taken.unwrap()] ^= 1 << new_square;
        }
    }

    //helper function for move_piece
    //updates all relevant postition bit boards
    fn update_postion_bb(&mut self, old_square: Square, new_square: Square, piece_moved: Piece, piece_taken: Option<Piece>) {
        self.position_bb[piece_moved.get_color()] |= 1 << new_square;
        self.position_bb[Color::Both] |= 1 << new_square;

        self.position_bb[piece_moved.get_color()] ^= 1 << old_square;
        self.position_bb[Color::Both] ^= 1 << old_square;

        if Option::is_some(&piece_taken) {
            self.position_bb[piece_taken.unwrap().get_color()] ^= 1 << new_square as u64;
        }
    }

    //Prints formant board possition
    pub fn print_board(&self) {
        println!("   A    B    C    D    E    F    G    H");

        for rank in (0..8).rev() {
            print!("{}", rank + 1);

            for file in 0..8 {
                let sqaure = get_square(rank, file);
                let mut piece: char = '-';

                for p in 0..12 {
                    if is_occupied(self.piece_bb[p], sqaure) {
                        piece = PIECE_CHARS_FANCY[p];
                        break;
                    }
                }

                print!("| {} |", piece);
            }
            println!();
        }
        println!();
    }

    //Displays info about the board state.
    //Prints current board possition.
    pub fn display_info(&self) {
        println!("--------------------");
        println!("Side: {:?}", self.side);
        println!("Enpas: {:?}", self.enpas);
        println!("Cast Perm: {}", self.cast_perm);
        println!("Fifty Move: {}", self.half_move);
        println!("Full Moves: {}", self.full_move);
        println!("--------------------");
        self.print_board();
    }

    pub fn print_all(&self) {
        let mut x = 0;
        for bb in self.piece_bb {
            print!("Piece: {}", PIECE_CHARS[x]);
            print_bb(bb);
            x += 1;
        }

        for bb in self.position_bb {
            print_bb(bb);
        }
    }
}

//Builds the board state from FEN string.
//Parses all fen parts an inits them to the board state.
//Returns a Board.
fn build_board(fen: &str) -> Board {
    let fen_parts: Vec<&str> = fen.split_whitespace().collect();
    let bb_tuple = parse_piece_placment(fen_parts[0]);
    return Board {
        piece_bb: bb_tuple.0,
        position_bb: bb_tuple.1,
        side: parse_side(fen_parts[1]),
        cast_perm: parse_castle(fen_parts[2]),
        enpas: parse_enpas(fen_parts[3]),
        half_move: fen_parts[4].parse::<u8>().unwrap(),
        full_move: fen_parts[5].parse::<u32>().unwrap(),
    };
}

//Parses piece placement data from FEN
fn parse_piece_placment(fen_pieces: &str) -> ([u64; 12], [u64; 3]) {
    let mut file: u8 = 0;
    let mut rank: u8 = 7;

    let mut piece_bb: [u64; 12] = [0; 12];
    let mut position_bb: [u64; 3] = [0; 3];

    for fen_char in fen_pieces.chars() {
        match fen_char {
            'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                let piece = Piece::from_char(fen_char);
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), piece, piece.get_color());
                file += 1;
            }
            '1' => file += 1,
            '2' => file += 2,
            '3' => file += 3,
            '4' => file += 4,
            '5' => file += 5,
            '6' => file += 6,
            '7' => file += 7,
            '8' => file += 8,
            '/' => continue,
            _ => todo!("throw error for improper fen"),
        }

        if file == 8 && rank > 0 {
            rank -= 1;
        }

        file %= 8;
    }
    return (piece_bb, position_bb);
}

//Helper funtion for parse_pieces.
//Sets bits in all relevant bit boards for each piece.
fn init_square(piece_bb: &mut [u64; 12], position_bb: &mut [u64; 3], square: Square, piece: Piece, color: Color) {
    piece_bb[piece] |= 1 << square;
    position_bb[color] |= 1 << square;
    position_bb[Color::Both] |= 1 << square;
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
            'K' => cast_perm += WKC,
            'Q' => cast_perm += WQC,
            'k' => cast_perm += BKC,
            'q' => cast_perm += BQC,
            _ => continue,
        }
    }
    return cast_perm;
}

//parses enpas fen string and sets state
fn parse_enpas(fen_enpas: &str) -> Option<Square> {
    if fen_enpas.len() == 1 {
        return None;
    }
    let c = fen_enpas.as_bytes();

    return Some(Square::from_u8((((c[0] - 96) % 9) * (c[1] - 48)) - 1));
}
