use rand::Rng;
use std::cmp::Ordering;

struct GameState {
    secret: u32,
    from_range: u32,
    to_range: u32,
    tries: u32,
    playing: bool,
}

impl GameState {
    fn increment_tries(&mut self) {
        self.tries += 1
    }

    fn update_from_range(&mut self, guess: u32) {
        self.from_range = if guess > self.from_range {
            guess + 1
        } else {
            self.from_range
        }
    }

    fn update_to_range(&mut self, guess: u32) {
        self.to_range = if guess < self.to_range {
            guess - 1
        } else {
            self.to_range
        }
    }
}

fn main() {
    let mut game_state = GameState {
        secret: gen_secret(1, 101),
        from_range: 1,
        to_range: 100,
        tries: 0,
        playing: true,
    };

    while game_state.playing {
        let guess = guess_number(game_state.from_range, game_state.to_range);
        let comparison = guess.cmp(&game_state.secret);
        match comparison {
            Ordering::Less => update_less(&mut game_state, guess),
            Ordering::Greater => update_greater(&mut game_state, guess),
            Ordering::Equal => update_equal(&mut game_state),
        }
    }

    println!("Found {} in {} tries", game_state.secret, game_state.tries);
}

fn update_less(game_state: &mut GameState, guess: u32) {
    game_state.increment_tries();
    game_state.update_from_range(guess);
    println!("Too small!");
}

fn update_greater(game_state: &mut GameState, guess: u32) {
    game_state.increment_tries();
    game_state.update_to_range(guess);
    println!("Too big!");
}

fn update_equal(game_state: &mut GameState) {
    game_state.increment_tries();
    println!("Found it!");
    game_state.playing = false;
}

fn gen_secret(from: u32, to: u32) -> u32 {
    let secret = rand::thread_rng().gen_range(from, to);
    println!("Debug: Secret is {}", secret);
    return secret;
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
