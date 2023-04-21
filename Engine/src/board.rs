use crate::util::{Color, Piece, get_square, Square, CastlingRights, is_occupied, INT_TO_CHAR};

pub struct Board{
    pub piece_bb:[u64;12],
    pub position_bb:[u64;3],

    side:Color,
    enpas:Option<Square>,
    cast_perm:u8,
    half_move:u32,
    full_move:u32,

    fifty_move:u32,
}

impl Default for Board{
    fn default() -> Self {
        Board { 
            piece_bb: [0;12], 
            position_bb: [0;3], 
            side: Color::WHITE, 
            enpas: None, 
            cast_perm: 0, 
            half_move: 0, 
            full_move: 0,
            fifty_move: 0,  
        }
    }
}

impl Board{
    pub fn init(fen:Option<&str>) -> Board{
        let mut board = Board::default();
        if fen.is_some(){
            board.build_board(fen.unwrap());
            return board;
        }
        
        board.build_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        return board;
    }
    
    fn set_square(&mut self, square:Square, piece:Piece, color:Color){
        self.piece_bb[piece as usize] = self.piece_bb[piece] | (1 << square as u64);
        self.position_bb[color as usize] = self.position_bb[color] | (1 << square as u64);
        self.position_bb[Color::BOTH as usize] = self.position_bb[Color::BOTH as usize] | (1 << square as u64);
    }
    
    //Splits FEN into parts and sets board state 
    pub fn build_board(&mut self, fen:&str){
        let fen_parts:Vec<&str> = fen.split_whitespace().collect();
        self.parse_pieces(fen_parts[0]);
        self.parse_side(fen_parts[1]);
        self.parse_castle(fen_parts[2]);
        self.parse_enpas(fen_parts[3]);
        self.half_move = fen_parts[4].parse().unwrap();
        self.full_move = fen_parts[5].parse().unwrap();
    }

    //perses possition fen string and sets board state
    fn parse_pieces(&mut self, fen_pieces:&str){
        let mut file:u8 = 0; 
        let mut rank:u8 = 7;
        
        for fen_char in fen_pieces.chars(){
            match fen_char{
                'p' =>  {
                    self.set_square(get_square(rank, file), Piece::BP, Color::BLACK); 
                    file += 1;
                },
                'b' => {
                    self.set_square(get_square(rank, file), Piece::BB, Color::BLACK); 
                    file += 1;
                },
                'n' => {
                    self.set_square(get_square(rank, file), Piece::BN, Color::BLACK); 
                    file += 1;
                },
                'r' => {
                    self.set_square(get_square(rank, file), Piece::BR, Color::BLACK);  
                    file += 1;
                },
                'q' => {
                    self.set_square(get_square(rank, file), Piece::BQ, Color::BLACK);  
                    file += 1;
                },
                'k' => {
                    self.set_square(get_square(rank, file), Piece::BQ, Color::BLACK);  
                    file += 1;
                },
                'P' => {
                    self.set_square(get_square(rank, file), Piece::WP, Color::WHITE);  
                    file += 1;
                },
                'B' => {
                    self.set_square(get_square(rank, file), Piece::WB, Color::WHITE);   
                    file += 1;
                },
                'N' => {
                    self.set_square(get_square(rank, file), Piece::WN, Color::WHITE);   
                    file += 1;
                },
                'R' => {
                    self.set_square(get_square(rank, file), Piece::WR, Color::WHITE);   
                    file += 1;
                },
                'Q' => {
                    self.set_square(get_square(rank, file), Piece::WQ, Color::WHITE);   
                    file += 1;
                },
                'K' => { 
                    self.set_square(get_square(rank, file), Piece::WK, Color::WHITE);  
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
    }

    //parses side fen string and sets state
    fn parse_side(&mut self, fen_side:&str){
        if fen_side.eq("b"){
            self.side = Color::BLACK;
        }
        else{
            self.side = Color::WHITE;
        }
    }
    
    //parses castle permistion fen string and sets state
    fn parse_castle(&mut self, fen_castle:&str){
        for fen_char in fen_castle.chars(){
            match fen_char {
                'K' => self.cast_perm += CastlingRights::WKC as u8,
                'Q' => self.cast_perm += CastlingRights::WQC as u8,
                'k' => self.cast_perm += CastlingRights::BKC as u8,
                'q' => self.cast_perm += CastlingRights::BQC as u8,
                _ => ()
            }
        }
    }

    //parses enpas fen string and sets state
    fn parse_enpas(&mut self, fen_enpas:&str){
        if fen_enpas.len() == 1{
            self.enpas = None;
        }
        else{
            self.enpas = None;
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
        println!("Side: {:#?}", self.side);
        println!("Enpas: {:#?}", self.enpas);
        println!("Cast Perm: {}", self.cast_perm);
        println!("Half Moves: {}", self.half_move);
        println!("Full Moves: {}", self.full_move);
        println!("--------------------");
        self.print_board();
    }
}

