use board_info::board_state::BoardState;

mod board_info;
mod move_gen;
mod util;

fn main() {
    let b = BoardState::init(None);

    b.display_info();
}
