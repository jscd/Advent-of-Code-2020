use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "day2-1.input";


fn main() {
  let input = File::open(INPUT_PATH).unwrap();
  let buffered = BufReader::new(input);

  let mut correct = 0;

  for password_line in buffered.lines() {
    let password_line = password_line.unwrap();
    let parts: Vec<&str> = password_line.split(":").collect();

    let password = parts[1].trim();
    let (limiter, p1, p2) = parse_constraints(&parts[0]);

    let password_vec: Vec<char> = password.chars().collect();

    let (p1, p2): (usize, usize) = ((p1-1) as usize, (p2-1) as usize);

    let (c1, c2) = (password_vec[p1], password_vec[p2]);


    let is_valid = check_constraints(&c1, &c2, &limiter);

    if is_valid {
      correct += 1;
    }

  }

  println!("correct: {}", correct);
}


fn parse_constraints(cons: &str) -> (char, u32, u32) {
  let parts: Vec<&str> = cons.split(" ").collect();

  let limits: Vec<&str> = parts[0].split("-").collect();
  let limits: (u32, u32) = (limits[0].parse().unwrap(), limits[1].parse().unwrap());

  let ch: Vec<char> = parts[1].trim().chars().collect();
  let ch: char = ch[0];

  (ch, limits.0, limits.1)
}

fn check_constraints(c1: &char, c2: &char, limiter: &char) -> bool {
  if c1 == c2 { return false; }

  if c1 == limiter || c2 == limiter {
    return true;
  }

  false
}
