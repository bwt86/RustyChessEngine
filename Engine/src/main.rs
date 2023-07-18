mod core;
mod game_logic;
mod move_logic;

fn main() {
    let mut game_state = game_logic::game::GameState::new(None);

    game_state.run();
}
