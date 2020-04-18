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
    fn new(from: u32, to: u32) -> GameState {
        GameState {
            secret: gen_secret(from, to + 1),
            from_range: from,
            to_range: to,
            tries: 0,
            playing: true,
        }
    }

    fn make_guess(&mut self, guess: u32) {
        self.increment_tries();
        let comparison = guess.cmp(&self.secret);
        match comparison {
            Ordering::Less => self.update_less(guess),
            Ordering::Greater => self.update_greater(guess),
            Ordering::Equal => self.update_equal(),
        }
    }

    fn increment_tries(&mut self) {
        self.tries += 1
    }

    fn update_less(&mut self, guess: u32) {
        self.update_from_range(guess);
        println!("Too small!");
    }
    fn update_greater(&mut self, guess: u32) {
        self.update_to_range(guess);
        println!("Too big!");
    }

    fn update_equal(&mut self) {
        println!("Found it!");
        self.playing = false;
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
    let mut game_state = GameState::new(1, 100);

    while game_state.playing {
        let guess = guess_number(game_state.from_range, game_state.to_range);
        game_state.make_guess(guess);
    }

    println!("Found {} in {} tries", game_state.secret, game_state.tries);
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
