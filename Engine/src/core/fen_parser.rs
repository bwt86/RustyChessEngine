use super::{
    bitboard::Bitboard,
    board_state::BoardState,
    piece::{CastlePerms, Color, Piece},
    piece_square_table::*,
    square::{File, Rank, Square},
    zobrist::ZobristHasher,
};

/// Parses a FEN string into a BoardState
///
/// # Arguments
/// * `fen` - The FEN string to parse
/// * `zobrist` - The Zobrist hasher for position hashing
///
/// # Returns
/// A Result containing either the parsed BoardState or a String error message
#[inline]
pub fn parse_fen(fen: &str, zobrist: &ZobristHasher) -> Result<BoardState, String> {
    let fen_parts: Vec<&str> = fen.split_whitespace().collect();

    if fen_parts.len() != 6 {
        return Err(format!("Invalid FEN string: expected 6 parts, got {}", fen_parts.len()));
    }

    let mut piece_bb = [Bitboard::new_empty(); 12];
    let mut position_bb = [Bitboard::new_empty(); 2];
    let mut board = [None; 64];
    let mut piece_lists: [Vec<Square>; 12] = Default::default();
    let mut material: [i32; 2] = [0; 2];
    let mut piece_counts: [u8; 12] = [0; 12];

    // Parse piece placement
    parse_piece_placement(
        fen_parts[0],
        &mut piece_bb,
        &mut position_bb,
        &mut board,
        &mut piece_lists,
        &mut material,
        &mut piece_counts,
    )?;

    // Parse side to move
    let side = parse_side(fen_parts[1])?;

    // Parse castling rights
    let castling_rights = parse_castling_rights(fen_parts[2])?;

    // Parse en passant square
    let en_passant = parse_en_passant(fen_parts[3])?;

    // Parse move counters
    let (half_moves, full_moves) = parse_move_counters(fen_parts[4], fen_parts[5])?;

    let psqt = PieceSquareTable::new();

    Ok(BoardState::init(
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
    ))
}

/// Parses the piece placement part of a FEN string
#[inline]
fn parse_piece_placement(
    fen: &str,
    piece_bb: &mut [Bitboard; 12],
    position_bb: &mut [Bitboard; 2],
    board: &mut [Option<Piece>; 64],
    piece_list: &mut [Vec<Square>; 12],
    material: &mut [i32; 2],
    piece_count: &mut [u8; 12],
) -> Result<(), String> {
    let mut file_index: usize = 0;
    let mut rank_index: usize = 7;

    for piece_char in fen.chars() {
        match piece_char {
            'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                if file_index >= 8 {
                    return Err("Invalid piece placement: too many pieces on rank".into());
                }
                let piece = Piece::from_char(piece_char).map_err(|_| format!("Invalid piece placement: invalid piece character: {}", piece_char))?;
                let sq = Square::from_file_rank(File::from_index(file_index), Rank::from_index(rank_index));
                init_square(piece_bb, position_bb, board, piece_list, sq, piece, piece.get_color());
                material[piece.get_color()] += piece.get_value();
                piece_count[piece] += 1;
                file_index += 1;
            }
            '1'..='8' => {
                let num = piece_char.to_digit(10).unwrap() as usize;
                if file_index + num > 8 {
                    return Err("Invalid piece placement: invalid empty squares count".into());
                }
                file_index += num;
            }
            '/' => {
                if file_index != 8 {
                    return Err("Invalid piece placement: invalid rank length".into());
                }
                file_index = 0;
                if rank_index == 0 {
                    return Err("Invalid piece placement: too many ranks".into());
                }
                rank_index -= 1;
            }
            _ => return Err(format!("Invalid piece placement: invalid character: {}", piece_char)),
        }
    }

    if rank_index != 0 || file_index != 8 {
        return Err("Invalid piece placement: incomplete board".into());
    }

    Ok(())
}

/// Initializes a square with a piece
#[inline(always)]
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

/// Parses the side to move from a FEN string
#[inline(always)]
fn parse_side(fen_side: &str) -> Result<Color, String> {
    match fen_side {
        "w" => Ok(Color::White),
        "b" => Ok(Color::Black),
        _ => Err(format!("Invalid side to move: {}", fen_side)),
    }
}

/// Parses the castling rights from a FEN string
#[inline(always)]
fn parse_castling_rights(fen_castle: &str) -> Result<u8, String> {
    if fen_castle == "-" {
        return Ok(0);
    }

    let mut cast_perm: u8 = 0;
    for fen_char in fen_castle.chars() {
        match fen_char {
            'K' => cast_perm |= CastlePerms::WKC as u8,
            'Q' => cast_perm |= CastlePerms::WQC as u8,
            'k' => cast_perm |= CastlePerms::BKC as u8,
            'q' => cast_perm |= CastlePerms::BQC as u8,
            _ => return Err(format!("Invalid castling rights: invalid character: {}", fen_char)),
        }
    }
    Ok(cast_perm)
}

/// Parses the en passant square from a FEN string
#[inline(always)]
fn parse_en_passant(fen_enpas: &str) -> Result<Option<Square>, String> {
    if fen_enpas == "-" {
        return Ok(None);
    }
    Square::from_string(fen_enpas)
        .map(Some)
        .map_err(|_| format!("Invalid en passant square: {}", fen_enpas))
}

/// Parses the move counters from a FEN string
#[inline(always)]
fn parse_move_counters(half_moves: &str, full_moves: &str) -> Result<(u8, u32), String> {
    let half = half_moves
        .parse::<u8>()
        .map_err(|_| format!("Invalid half-move counter: {}", half_moves))?;
    let full = full_moves
        .parse::<u32>()
        .map_err(|_| format!("Invalid full-move counter: {}", full_moves))?;
    Ok((half, full))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fen() {
        let zobrist = ZobristHasher::new();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let result = parse_fen(fen, &zobrist);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_fen() {
        let zobrist = ZobristHasher::new();
        let fen = "invalid fen";
        let result = parse_fen(fen, &zobrist);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_piece_placement() {
        let zobrist = ZobristHasher::new();
        let fen = "rnbqkbnr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let result = parse_fen(fen, &zobrist);
        assert!(result.is_err());
    }
}
