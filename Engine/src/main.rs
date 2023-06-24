mod core;
mod game;
mod move_gen;
fn main() {
    let g = game::Game::new();

    g.get_board_state().display_info();
}
