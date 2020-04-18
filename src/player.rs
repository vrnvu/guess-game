pub fn guess_number(from_range: u32, to_range: u32) -> u32 {
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
