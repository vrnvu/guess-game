use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret = gen_secret(1, 101);

    let mut from_range = 1;
    let mut to_range = 100;
    let mut tries = 0;

    loop {
        tries += 1;
        let guess = guess_number(from_range, to_range);
        let comparison = guess.cmp(&secret);
        match comparison {
            Ordering::Less => {
                from_range = update_from_range(from_range, guess);
                println!("Too small!");
            }
            Ordering::Greater => {
                to_range = update_to_range(to_range, guess);
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("Found it!");
                break;
            }
        }
    }
    println!("Found {} in {} tries", secret, tries);
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
