mod game;
mod player;

fn main() {
    let mut game_state = game::new(1, 100);

    while game::is_playing(&game_state) {
        let from = game::get_from(&game_state);
        let to = game::get_to(&game_state);
        let guess = player::guess_number(from, to);
        game_state.make_guess(guess);
    }

    game::finish(game_state);
}
