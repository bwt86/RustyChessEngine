use super::attack_pregen::PregenAttacks;
use super::fen_parser::parse_fen;
use super::piece_square_table::Phase;
use super::piece_square_table::PieceSquareTable;
use super::zobrist::ZobristHasher;
use crate::core::bitboard::*;
use crate::core::piece::*;
use crate::core::square::*;
use crate::move_logic::move_encode::Move;

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, PartialEq, Clone)]
pub struct BoardState {
    // Board representations
    piece_bb: [Bitboard; 12],       // Bitboards for each piece
    position_bb: [Bitboard; 2],     // Bitboards for each color
    board: [Option<Piece>; 64],     // Array representing the board
    piece_lists: [Vec<Square>; 12], // Piece lists for each piece

    // Other game state
    side: Color,                // Side to move next
    en_passant: Option<Square>, // En passant square, if any
    castling_rights: u8,        // Castling rights for each side (KQkq)
    half_moves: u8,             // Half-moves since last pawn move or capture
    full_moves: u32,            // Total full moves in the game

    // Evaluation information
    material: [i32; 2],     // Material value for each side
    piece_counts: [u8; 12], // Count of each type of piece
    psqt: PieceSquareTable, // Piece-square tables for each piece

    // Search information
    zobrist_hash: u64,     // Zobrist hash of the position
    pawn_hash: u64,        // Separate Zobrist hash for pawn structure
    king_safety: [i32; 2], // King safety scores for each side
}

impl BoardState {
    pub fn new(fen_str: Option<&str>, zobrist: &ZobristHasher) -> BoardState {
        parse_fen(fen_str.unwrap_or(DEFAULT_FEN), zobrist)
    }

    pub fn init(
        piece_bb: [Bitboard; 12],
        position_bb: [Bitboard; 2],
        board: [Option<Piece>; 64],
        piece_lists: [Vec<Square>; 12],
        side: Color,
        en_passant: Option<Square>,
        castling_rights: u8,
        half_moves: u8,
        full_moves: u32,
        material: [i32; 2],
        piece_counts: [u8; 12],
        psqt: PieceSquareTable,
        zobrist_hash: u64,
        pawn_hash: u64,
        king_safety: [i32; 2],
    ) -> BoardState {
        BoardState {
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
            zobrist_hash,
            pawn_hash,
            king_safety,
        }
    }

    pub fn get_board(&self) -> &[Option<Piece>; 64] {
        &self.board
    }

    pub fn get_piece_bb(&self, piece: Piece) -> Bitboard {
        self.piece_bb[piece]
    }

    pub fn get_position_bb(&self, color: Color) -> Bitboard {
        self.position_bb[color]
    }

    pub fn get_combined_bb(&self) -> Bitboard {
        self.position_bb[0].combine(self.position_bb[1])
    }

    pub fn get_piece_lists(&self) -> &[Vec<Square>; 12] {
        &self.piece_lists
    }

    pub fn get_piece_squares(&self, piece: Piece) -> &[Square] {
        &self.piece_lists[piece]
    }

    pub fn get_side(&self) -> Color {
        self.side
    }

    pub fn get_opposite_side(&self) -> Color {
        self.side.opposite()
    }

    pub fn get_en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    pub fn get_castling_rights(&self) -> u8 {
        self.castling_rights
    }

    pub fn check_castle(&self, castling: u8) -> bool {
        return self.castling_rights & castling == castling;
    }

    pub fn get_half_moves(&self) -> u8 {
        self.half_moves
    }

    pub fn get_full_moves(&self) -> u32 {
        self.full_moves
    }

    pub fn get_material(&self) -> &[i32; 2] {
        &self.material
    }

    pub fn get_material_difference(&self) -> i32 {
        self.material[Color::White] - self.material[Color::Black]
    }

    pub fn get_white_material(&self) -> i32 {
        self.material[0]
    }

    pub fn get_black_material(&self) -> i32 {
        self.material[1]
    }

    pub fn get_piece_counts(&self) -> &[u8; 12] {
        &self.piece_counts
    }

    pub fn get_piece_count(&self, piece: Piece) -> u8 {
        self.piece_counts[piece as usize]
    }

    pub fn get_num_pieces(&self) -> u8 {
        self.piece_counts.iter().sum()
    }

    pub fn get_zobrist_hash(&self) -> u64 {
        self.zobrist_hash
    }

    pub fn get_pawn_hash(&self) -> u64 {
        self.pawn_hash
    }

    pub fn get_king_safety(&self) -> &[i32; 2] {
        &self.king_safety
    }

    pub fn get_white_king_safety(&self) -> i32 {
        self.king_safety[0]
    }

    pub fn get_black_king_safety(&self) -> i32 {
        self.king_safety[1]
    }

    pub fn get_piece_on_square(&self, sq: Square) -> Option<Piece> {
        return self.board[sq];
    }

    pub fn get_psq_value(&self, piece: Piece, sq: Square) -> i32 {
        let phase = Phase::Opening;
        let piece_type = piece.get_type();
        let piece_color = piece.get_color();

        self.psqt.get_psq_value(phase, piece, sq)
    }

    pub fn make_null_move(&mut self, zobrist: &ZobristHasher) {
        self.side = self.side.opposite();
        zobrist.update_zobrist_hash_side(&mut self.zobrist_hash);
    }

    pub fn unmake_null_move(&mut self, zobrist: &ZobristHasher) {
        self.side = self.side.opposite();
        zobrist.update_zobrist_hash_side(&mut self.zobrist_hash);
    }

    pub fn make_move(&mut self, c_move: Move, zobrist: &ZobristHasher) {
        let piece = c_move.get_piece();
        let from = c_move.get_from();
        let to = c_move.get_to();
        let promotion = c_move.get_promotion();
        let is_en_passant = c_move.is_en_passant();
        let is_castle = c_move.is_castling();
        let is_double_push = c_move.is_double_pawn_push();

        self.update_bitboards(piece, from, Some(to));
        self.update_board(piece, from, to);
        self.update_piece_lists(piece, from, Some(to));

        zobrist.update_zobrist_hash_move(&mut self.zobrist_hash, &mut self.pawn_hash, piece, from, to);

        if let Some(captured_piece) = c_move.get_capture() {
            if is_en_passant {
                let sq = match self.get_opposite_side() {
                    Color::White => self.en_passant.unwrap().move_up(1),
                    Color::Black => self.en_passant.unwrap().move_down(1),
                };

                self.update_bitboards(captured_piece, sq, None);
                self.board[sq] = None;
                self.update_piece_lists(captured_piece, sq, None);
                zobrist.update_zobrist_hash_capture(&mut self.zobrist_hash, &mut self.pawn_hash, captured_piece, sq);
            } else {
                self.update_bitboards(captured_piece, to, None);
                self.update_piece_lists(captured_piece, to, None);
                zobrist.update_zobrist_hash_capture(&mut self.zobrist_hash, &mut self.pawn_hash, captured_piece, to);
            }

            self.piece_counts[captured_piece] -= 1;
            self.material[captured_piece.get_color()] -= captured_piece.get_value();
        }

        if is_castle {
            let rook = Piece::new(piece.get_color(), PieceType::Rook);
            let rook_from = match to {
                Square::C1 => Square::A1,
                Square::C8 => Square::A8,
                Square::G1 => Square::H1,
                Square::G8 => Square::H8,
                _ => panic!("Invalid castle move"),
            };
            let rook_to: Square = match to {
                Square::C1 => Square::D1,
                Square::C8 => Square::D8,
                Square::G1 => Square::F1,
                Square::G8 => Square::F8,
                _ => panic!("Invalid castle move"),
            };

            self.update_bitboards(rook, rook_from, Some(rook_to));
            self.update_board(rook, rook_from, rook_to);
            self.update_piece_lists(rook, rook_from, Some(rook_to));

            zobrist.update_zobrist_hash_move(&mut self.zobrist_hash, &mut self.pawn_hash, rook, rook_from, rook_to);
        }

        //update castling rights
        if piece.get_type() == PieceType::King {
            let old_rights = self.castling_rights;
            if self.side == Color::White {
                self.castling_rights &= 0b1100;
            } else {
                self.castling_rights &= 0b0011;
            }
            zobrist.update_zobrist_hash_castling(&mut self.zobrist_hash, old_rights, self.castling_rights);
        } else if piece.get_type() == PieceType::Rook {
            let rook_start_squares: [Square; 4] = [Square::A1, Square::H1, Square::A8, Square::H8];
            for (i, &sq) in rook_start_squares.iter().enumerate() {
                if c_move.get_from() == sq {
                    let old_rights = self.castling_rights;
                    self.castling_rights &= !(1 << i);
                    zobrist.update_zobrist_hash_castling(&mut self.zobrist_hash, old_rights, self.castling_rights);
                    break;
                }
            }
        }

        if is_double_push {
            zobrist.update_zobrist_hash_en_passant(&mut self.zobrist_hash, self.en_passant, Some(from));
            self.en_passant = match piece.get_color() {
                Color::White => Some(to.move_down(1)),
                Color::Black => Some(to.move_up(1)),
            };
        } else {
            zobrist.update_zobrist_hash_en_passant(&mut self.zobrist_hash, self.en_passant, None);
            self.en_passant = None;
        }

        if let Some(promotion) = promotion {
            let pawn = Piece::new(piece.get_color(), PieceType::Pawn);
            self.update_bitboards(pawn, to, None);
            self.update_piece_lists(pawn, to, None);

            self.update_bitboards(promotion, to, Some(to));
            self.update_board(promotion, to, to);
            self.update_piece_lists(promotion, to, Some(to));

            self.piece_counts[pawn] -= 1;
            self.material[pawn.get_color()] -= pawn.get_value();
            self.piece_counts[promotion] += 1;
            self.material[promotion.get_color()] += promotion.get_value();

            zobrist.update_zobrist_hash_promotion(&mut self.zobrist_hash, &mut self.pawn_hash, pawn, promotion, to);
        }

        if piece.get_type() == PieceType::Pawn || c_move.is_capture() {
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }

        if self.side == Color::Black {
            self.full_moves += 1;
        }

        self.side = self.side.opposite();
    }

    fn update_bitboards(&mut self, piece: Piece, from: Square, to: Option<Square>) {
        match to {
            Some(to) => {
                self.piece_bb[piece].make_move(from, to);
                self.position_bb[piece.get_color()].make_move(from, to);
            }
            None => {
                self.piece_bb[piece].clear_square(from);
                self.position_bb[piece.get_color()].clear_square(from);
            }
        }
    }

    fn update_board(&mut self, piece: Piece, from: Square, to: Square) {
        self.board[from] = None;
        self.board[to] = Some(piece);
    }

    fn update_piece_lists(&mut self, piece: Piece, from: Square, to: Option<Square>) {
        if let Some(from_index) = self.piece_lists[piece].iter().position(|&x| x == from) {
            self.piece_lists[piece].remove(from_index);
        }

        if let Some(to) = to {
            self.piece_lists[piece].push(to);
        }
    }

    pub fn get_hanging_bb(&self, side: Color, pregen_attacks: &PregenAttacks) -> Bitboard {
        let enemy_color = side.opposite();
        let combined_bb = &self.get_combined_bb();

        let mut enemy_attack_bb = Bitboard::new_empty();
        let mut my_defense_bb = Bitboard::new_empty();

        for piece_type in PIECE_TYPES {
            let enemy_piece = Piece::new(enemy_color, piece_type);
            let my_piece = Piece::new(side, piece_type);

            let enemy_piece_squares = &self.piece_lists[enemy_piece];
            let my_piece_squares = &self.piece_lists[my_piece];

            for &sq in enemy_piece_squares {
                let attacks = match piece_type {
                    PieceType::Pawn => pregen_attacks.get_pawn_attacks(enemy_color, sq),
                    PieceType::Knight => pregen_attacks.get_knight_attacks(sq),
                    PieceType::Bishop => pregen_attacks.get_bishop_attacks(sq, combined_bb),
                    PieceType::Rook => pregen_attacks.get_rook_attacks(sq, combined_bb),
                    PieceType::Queen => pregen_attacks.get_queen_attacks(sq, combined_bb),
                    PieceType::King => pregen_attacks.get_king_attacks(sq),
                };

                enemy_attack_bb = enemy_attack_bb.combine(attacks);
            }

            for &sq in my_piece_squares {
                let defenses = match piece_type {
                    PieceType::Pawn => pregen_attacks.get_pawn_attacks(side, sq),
                    PieceType::Knight => pregen_attacks.get_knight_attacks(sq),
                    PieceType::Bishop => pregen_attacks.get_bishop_attacks(sq, combined_bb),
                    PieceType::Rook => pregen_attacks.get_rook_attacks(sq, combined_bb),
                    PieceType::Queen => pregen_attacks.get_queen_attacks(sq, combined_bb),
                    PieceType::King => pregen_attacks.get_king_attacks(sq),
                };

                my_defense_bb = my_defense_bb.combine(defenses);
            }
        }

        enemy_attack_bb = enemy_attack_bb.intersect(self.position_bb[side]);
        enemy_attack_bb
            .intersect(my_defense_bb.invert())
            .intersect(self.piece_bb[Piece::new(side, PieceType::King)].invert())
    }

    pub fn count_double_pawns(&self, side: Color) -> i32 {
        let mut double_pawns = 0;
        let piece = Piece::new(side, PieceType::Pawn);

        for file in FILES_BB {
            let pawns_on_file = self.piece_bb[piece].intersect(file);

            if pawns_on_file.count_squares() > 1 {
                double_pawns += 1;
            }
        }

        double_pawns
    }

    pub fn is_check(&self, side: Color, pregen_attacks: &PregenAttacks) -> bool {
        let king_sq = self.piece_lists[Piece::new(side, PieceType::King)][0];
        let enemy_color = side.opposite();

        for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen] {
            let piece = Piece::new(enemy_color, piece_type);
            let piece_squares = &self.piece_lists[piece];

            for &sq in piece_squares {
                let attacks = match piece_type {
                    PieceType::Pawn => pregen_attacks.get_pawn_attacks(enemy_color, sq),
                    PieceType::Knight => pregen_attacks.get_knight_attacks(sq),
                    PieceType::Bishop => pregen_attacks.get_bishop_attacks(sq, &self.get_combined_bb()),
                    PieceType::Rook => pregen_attacks.get_rook_attacks(sq, &self.get_combined_bb()),
                    PieceType::Queen => pregen_attacks.get_queen_attacks(sq, &self.get_combined_bb()),
                    PieceType::King => continue,
                };

                if attacks.is_occupied(king_sq) {
                    return true;
                }
            }
        }
        false
    }

    pub fn evaluate(&self, pregen_attacks: &PregenAttacks) -> i32 {
        let mut score = self.get_material_difference();

        let side = self.side;
        let opposite_side = self.get_opposite_side();
        for (piece, sqs) in self.piece_lists.iter().enumerate() {
            let piece = Piece::from_index(piece);
            let color = piece.get_color();
            for &sq in sqs {
                let piece_value = piece.get_value();
                let psqt_bonus = self.get_psq_value(piece, sq);
                score += color.get_factor() * (piece_value + psqt_bonus);
            }
        }

        if self.is_check(side, &pregen_attacks) {
            score -= side.get_factor() * 100
        }

        if self.is_check(opposite_side, &pregen_attacks) {
            score += side.get_factor() * 100
        }

        // let hanging_bb = self.get_hanging_bb(side, &pregen_attacks);
        // for sq in hanging_bb.get_occupied_squares() {
        //     if let Some(piece) = self.board[sq] {
        //         let piece_value = piece.get_value();
        //         let factor = side.get_factor();
        //         score -= factor * piece_value;
        //     }
        // }

        // let enemy_hanging = self.get_hanging_bb(opposite_side, &pregen_attacks);
        // for sq in enemy_hanging.get_occupied_squares() {
        //     if let Some(piece) = self.board[sq] {
        //         let piece_value = piece.get_value();
        //         let factor = side.get_factor();
        //         score += factor * piece_value;
        //     }
        // }

        score -= side.get_factor() * 10 * self.count_double_pawns(side);
        score += opposite_side.get_factor() * 10 * self.count_double_pawns(opposite_side);

        score
    }

    pub fn print_board(&self) {
        println!("   A    B    C    D    E    F    G    H");

        for rank in RANKS.iter().rev() {
            print!("{}", (*rank as u8) + 1);

            for file in FILES.iter() {
                let sqaure = Square::from_file_rank(*file, *rank);
                let mut piece: char = '-';

                if let Some(p) = self.board[sqaure] {
                    piece = p.to_char_fancy();
                }

                print!("| {} |", piece);
            }
            println!();
        }
        println!();
    }

    pub fn display_info(&self, pregen_attacks: &PregenAttacks) {
        println!("--------------------");
        println!("Side: {:?}", self.side);
        println!("Enpas: {:?}", self.en_passant);
        println!("Cast Perm: {}", self.castling_rights);
        println!("Fifty Move: {}", self.half_moves);
        println!("Full Moves: {}", self.full_moves);
        println!("Zobrist Hash: {}", self.zobrist_hash);
        println!("Pawn Hash: {}", self.pawn_hash);
        println!("Eval: {}", self.evaluate(pregen_attacks));
        println!("--------------------");
        self.print_board();
    }
}
