use rand::Rng;
use std::cmp::Ordering;

pub struct State {
  secret: u32,
  from_range: u32,
  to_range: u32,
  tries: u32,
  playing: bool,
}

impl State {
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

pub fn new(from: u32, to: u32) -> State {
  State {
    secret: gen_secret(from, to + 1),
    from_range: from,
    to_range: to,
    tries: 0,
    playing: true,
  }
}

pub fn finish(state: State) {
  println!("Found {} in {} tries", state.secret, state.tries);
}

pub fn get_from(state: &State) -> u32 {
  state.from_range
}

pub fn get_to(state: &State) -> u32 {
  state.to_range
}

pub fn is_playing(state: &State) -> bool {
  state.playing
}

fn gen_secret(from: u32, to: u32) -> u32 {
  let secret = rand::thread_rng().gen_range(from, to);
  println!("Debug: Secret is {}", secret);
  return secret;
}
