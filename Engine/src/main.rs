use board::Board;
mod util;
mod board;

fn main() {
    let b = Board::init(None);
    b.display_info();
}
