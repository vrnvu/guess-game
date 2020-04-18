mod game;

fn main() {
    let mut game_state = game::GameState::new(1, 100);

    while game::is_playing(&game_state) {
        let from = game::get_from(&game_state);
        let to = game::get_to(&game_state);
        let guess = guess_number(from, to);
        game_state.make_guess(guess);
    }

    game::finish(game_state);
}

fn guess_number(from_range: u32, to_range: u32) -> u32 {
    println!(
        "Please enter a guess number between {} and {}:",
        from_range, to_range
    );
    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("failed reading input");

    let guess: u32 = guess.trim().parse().expect("Invalid number");

    println!("Your guess is {}", guess);
    return guess;
}
