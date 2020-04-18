use rand::Rng;
use std::cmp::Ordering;

struct GameState {
    secret: u32,
    from_range: u32,
    to_range: u32,
    tries: u32,
    playing: bool,
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
        game_state.tries += 1;
        let guess = guess_number(game_state.from_range, game_state.to_range);
        let comparison = guess.cmp(&game_state.secret);

        match comparison {
            Ordering::Less => {
                game_state.from_range = update_from_range(game_state.from_range, guess);
                println!("Too small!");
            }
            Ordering::Greater => {
                game_state.to_range = update_to_range(game_state.to_range, guess);
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("Found it!");
                game_state.playing = false;
            }
        }
    }

    println!("Found {} in {} tries", game_state.secret, game_state.tries);
}

fn update_from_range(from_range: u32, guess: u32) -> u32 {
    if guess > from_range {
        guess + 1
    } else {
        from_range
    }
}

fn update_to_range(to_range: u32, guess: u32) -> u32 {
    if guess < to_range {
        guess - 1
    } else {
        to_range
    }
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
