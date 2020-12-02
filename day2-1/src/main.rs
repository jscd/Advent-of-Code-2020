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
    let (limiter, min, max) = parse_constraints(&parts[0]);

    let mut l_count = 0;
    for ch in password.chars() {
      if ch == limiter {
        l_count += 1;
      }
    }

    let valid = (l_count >= min) && (l_count <= max);

    println!("'{}' has {} letter {}'s ({}-{}), thus is {}", password, l_count, limiter, min, max, if valid { "valid" } else { "invalid" });

    if valid {
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
