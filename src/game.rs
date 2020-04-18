use rand::Rng;
use std::cmp::Ordering;

pub struct GameState {
  secret: u32,
  from_range: u32,
  to_range: u32,
  tries: u32,
  playing: bool,
}

impl GameState {
  pub fn new(from: u32, to: u32) -> GameState {
    GameState {
      secret: gen_secret(from, to + 1),
      from_range: from,
      to_range: to,
      tries: 0,
      playing: true,
    }
  }

  pub fn make_guess(&mut self, guess: u32) {
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

pub fn finish(game_state: GameState) {
  println!("Found {} in {} tries", game_state.secret, game_state.tries);
}

pub fn get_from(game_state: &GameState) -> u32 {
  game_state.from_range
}

pub fn get_to(game_state: &GameState) -> u32 {
  game_state.to_range
}

pub fn is_playing(game_state: &GameState) -> bool {
  game_state.playing
}

fn gen_secret(from: u32, to: u32) -> u32 {
  let secret = rand::thread_rng().gen_range(from, to);
  println!("Debug: Secret is {}", secret);
  return secret;
}
