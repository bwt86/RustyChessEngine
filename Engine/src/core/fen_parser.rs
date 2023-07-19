use super::{
    bitboard::Bitboard,
    board_state::BoardState,
    piece::{CastlePerms, Color, Piece},
    piece_square_table::*,
    square::{File, Rank, Square},
    zobrist::ZobristHasher,
};

pub fn parse_fen(fen: &str, zobrist: &ZobristHasher) -> BoardState {
    let fen_parts: Vec<&str> = fen.split_whitespace().collect();

    if fen_parts.len() != 6 {
        panic!("Invalid FEN string: {}", fen);
    }

    let mut piece_bb = [Bitboard::new_empty(); 12];
    let mut position_bb = [Bitboard::new_empty(); 2];
    let mut board = [None; 64];
    let mut piece_lists: [Vec<Square>; 12] = Default::default();

    let side = parse_side(fen_parts[1]);
    let en_passant = parse_en_passant(fen_parts[3]);
    let castling_rights = parse_castling_rights(fen_parts[2]);
    let half_moves = fen_parts[4].parse::<u8>().unwrap();
    let full_moves = fen_parts[5].parse::<u32>().unwrap();

    let mut material: [i32; 2] = [0; 2];
    let mut piece_counts: [u8; 12] = [0; 12];
    let psqt = PieceSquareTable::init();

    parse_piece_placment(
        fen_parts[0],
        &mut piece_bb,
        &mut position_bb,
        &mut board,
        &mut piece_lists,
        &mut material,
        &mut piece_counts,
    );

    BoardState::init(
        piece_bb,
        position_bb,
        board,
        piece_lists,
        side,
        en_passant,
        castling_rights,
        half_moves,
        full_moves,
        material,
        piece_counts,
        psqt,
        zobrist.init_hash(&board, side, en_passant, castling_rights),
        zobrist.init_hash_pawns(&board),
        [0; 2],
    )
}

//Parses piece placement data from FEN
fn parse_piece_placment(
    fen: &str,
    piece_bb: &mut [Bitboard; 12],
    position_bb: &mut [Bitboard; 2],
    board: &mut [Option<Piece>; 64],
    piece_list: &mut [Vec<Square>; 12],
    material: &mut [i32; 2],
    piece_count: &mut [u8; 12],
) {
    let mut file_index: usize = 0;
    let mut rank_index: usize = 7;

    for piece_char in fen.chars() {
        match piece_char {
            'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                let piece = Piece::from_char(piece_char).unwrap();
                init_square(
                    piece_bb,
                    position_bb,
                    board,
                    piece_list,
                    Square::from_file_rank(File::from_index(file_index), Rank::from_index(rank_index)),
                    piece,
                    piece.get_color(),
                );
                material[piece.get_color()] += piece.get_value();
                piece_count[piece] += 1;
                file_index += 1;
            }
            '1'..='8' => {
                let num = piece_char.to_digit(10).unwrap() as usize;
                file_index += num;
            }
            '/' => {
                file_index = 0;
                rank_index -= 1;
            }
            _ => panic!("Invalid FEN string"),
        }
    }
}

//Helper funtion for parse_pieces.
//Sets bits in all relevant bit boards for each piece.
fn init_square(
    piece_bb: &mut [Bitboard; 12],
    position_bb: &mut [Bitboard; 2],
    board: &mut [Option<Piece>; 64],
    piece_list: &mut [Vec<Square>; 12],
    sq: Square,
    piece: Piece,
    color: Color,
) {
    piece_bb[piece].set_square(sq);
    position_bb[color].set_square(sq);

    board[sq] = Some(piece);
    piece_list[piece].push(sq);
}

//parses side fen string and sets state
fn parse_side(fen_side: &str) -> Color {
    if fen_side.eq("b") {
        return Color::Black;
    }
    return Color::White;
}

//parses castle permistion fen string and sets state
fn parse_castling_rights(fen_castle: &str) -> u8 {
    let mut cast_perm: u8 = 0;
    for fen_char in fen_castle.chars() {
        match fen_char {
            'K' => cast_perm |= CastlePerms::WKC as u8,
            'Q' => cast_perm |= CastlePerms::WQC as u8,
            'k' => cast_perm |= CastlePerms::BKC as u8,
            'q' => cast_perm |= CastlePerms::BQC as u8,
            _ => continue,
        }
    }
    cast_perm
}

//parses enpas fen string and sets state
fn parse_en_passant(fen_enpas: &str) -> Option<Square> {
    if fen_enpas.len() == 1 {
        return None;
    }
    Some(Square::from_string(fen_enpas).unwrap())
}
