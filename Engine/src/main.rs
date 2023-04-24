use board::Board;
mod board;
mod util;

fn main() {
    let mut b = Board::init(None);

    b.display_info();
    b.move_piece(
        util::Square::A2,
        util::Square::A7,
        util::Piece::WP,
        Some(util::Piece::BP),
    );
    b.display_info();
}
