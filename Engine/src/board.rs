
use crate::util::{Color, Piece, get_square, Square, CastlingRights, is_occupied, INT_TO_CHAR, print_bb};

pub struct Board{
    piece_bb:[u64;12],
    position_bb:[u64;3],

    side:Color,
    enpas:Option<Square>,
    cast_perm:u8,
    half_move:u32,
    full_move:u32,

    fifty_move:u32,
}

pub struct Move{
    piece_moved:Piece,
    piece_taken:Option<Piece>,
    old_square:Square,
    new_square:Square,
    prev_board_state:Board,
}

impl Board{
    pub fn init(fen:Option<&str>) -> Board{
        if fen.is_some(){
            return build_board(fen.unwrap());
        }
        
        return build_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq h8 0 1");
    }

    pub fn move_piece(&mut self, old_square:Square, new_square:Square, piece_moved:Piece, piece_taken:Option<Piece>){
        self.update_piece_bb(old_square, new_square, piece_moved, piece_taken);
        self.update_postion_bb(old_square, new_square, piece_moved, piece_taken);
    }

    fn update_piece_bb(&mut self, old_square:Square, new_square:Square, piece_moved:Piece, piece_taken:Option<Piece>){
        self.piece_bb[piece_moved] |= 1 << new_square;
        self.piece_bb[piece_moved] ^= 1 << old_square;

        if Option::is_some(&piece_taken) {
            self.piece_bb[piece_taken.unwrap()] ^= 1 << new_square; 
        }
    }

    fn update_postion_bb(&mut self, old_square:Square, new_square:Square, piece_moved:Piece, piece_taken:Option<Piece>){
        self.position_bb[Piece::get_color(piece_moved)] |= 1 << new_square; 
        self.position_bb[Color::BOTH] |= 1 << new_square;
        
        self.position_bb[Piece::get_color(piece_moved)] ^= 1 << old_square; 
        self.position_bb[Color::BOTH] ^= 1 << old_square;

        if Option::is_some(&piece_taken) {
            self.position_bb[Piece::get_color(piece_taken.unwrap())] ^= 1 << new_square as u64; 
        }
    }

    //Prints formant board possition
    pub fn print_board(&self){
        println!("   A    B    C    D    E    F    G    H");

        for rank in (0..8).rev(){
            print!("{}", rank + 1);
    
            for file in 0..8 {
    
                let sqaure = get_square(rank, file);
                let mut piece: char = '-';
    
                for p in 0..12 {
                    if is_occupied(self.piece_bb[p], sqaure){
                        piece = INT_TO_CHAR[p];
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
    pub fn display_info(&self){
        println!("--------------------");
        println!("Side: {:?}", self.side);
        println!("Enpas: {:?}", self.enpas);
        println!("Cast Perm: {}", self.cast_perm);
        println!("Half Moves: {}", self.half_move);
        println!("Full Moves: {}", self.full_move);
        println!("--------------------");
        self.print_board();
    }

    pub fn print_all(&self){
        let mut x = 0;
        for bb in self.piece_bb{
            print!("Piece: {}", INT_TO_CHAR[x]);
            print_bb(bb);
            x +=1;
        }
        
        for bb in self.position_bb{
            print_bb(bb);
        }
    }
}

//Splits FEN into parts and sets board state 
fn build_board(fen:&str) -> Board{
    let fen_parts:Vec<&str> = fen.split_whitespace().collect();
    let bb_tuple = parse_pieces(fen_parts[0]);
    return Board{
        piece_bb: bb_tuple.0,
        position_bb: bb_tuple.1,
        side: parse_side(fen_parts[1]),
        cast_perm: parse_castle(fen_parts[2]),
        enpas: parse_enpas(fen_parts[3]),
        half_move: fen_parts[4].parse().unwrap(),
        full_move: fen_parts[5].parse().unwrap(),
        fifty_move:0,
    };
}

fn init_square(piece_bb:&mut [u64;12], position_bb:&mut [u64;3], square:Square, piece:Piece, color:Color){
    piece_bb[piece] |= 1 << square;
    position_bb[color] |= 1 << square;
    position_bb[Color::BOTH] |= 1 << square;
}

//perses possition fen string and sets board state
fn parse_pieces(fen_pieces:&str) -> ([u64;12],[u64;3]){
    let mut file:u8 = 0; 
    let mut rank:u8 = 7;

    let mut piece_bb:[u64;12] = [0;12];
    let mut position_bb:[u64;3] = [0;3];
    
    for fen_char in fen_pieces.chars(){
        match fen_char{
            'p' =>  {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::BP, Color::BLACK); 
                file += 1;
            },
            'b' => {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::BB, Color::BLACK); 
                file += 1;
            },
            'n' => {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::BN, Color::BLACK); 
                file += 1;
            },
            'r' => {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::BR, Color::BLACK);  
                file += 1;
            },
            'q' => {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::BQ, Color::BLACK);  
                file += 1;
            },
            'k' => {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::BK, Color::BLACK);  
                file += 1;
            },
            'P' => {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::WP, Color::WHITE);  
                file += 1;
            },
            'B' => {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::WB, Color::WHITE);   
                file += 1;
            },
            'N' => {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::WN, Color::WHITE);   
                file += 1;
            },
            'R' => {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::WR, Color::WHITE);   
                file += 1;
            },
            'Q' => {
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::WQ, Color::WHITE);   
                file += 1;
            },
            'K' => { 
                init_square(&mut piece_bb, &mut position_bb, get_square(rank, file), Piece::WK, Color::WHITE);  
                file += 1;
            },
            '1' => file += 1,
            '2' => file += 2,
            '3' => file += 3,
            '4' => file += 4,
            '5' => file += 5,
            '6' => file += 6,
            '7' => file += 7,
            '8' => file += 8,
            _ => continue,
        }

        rank = if file == 8 && rank > 0 {rank - 1} else {rank};
        file %= 8;
    }
    return (piece_bb,position_bb);
}

//parses side fen string and sets state
fn parse_side(fen_side:&str) -> Color{
    if fen_side.eq("b"){
        return Color::BLACK;
    }
    return Color::WHITE;
}

//parses castle permistion fen string and sets state
fn parse_castle(fen_castle:&str) -> u8{
    let mut cast_perm:u8 = 0;
    for fen_char in fen_castle.chars(){
        match fen_char {
            'K' => cast_perm += CastlingRights::WKC as u8,
            'Q' => cast_perm += CastlingRights::WQC as u8,
            'k' => cast_perm += CastlingRights::BKC as u8,
            'q' => cast_perm += CastlingRights::BQC as u8,
            _ => continue
        }
    }
    return cast_perm;
}

//parses enpas fen string and sets state
fn parse_enpas(fen_enpas:&str) -> Option<Square>{
    if fen_enpas.len() == 1{
        return None;
    }
    let c = fen_enpas.as_bytes();

    return Some(Square::from_u8((((c[0] - 96) % 9) * (c[1] - 48)) - 1));
}

