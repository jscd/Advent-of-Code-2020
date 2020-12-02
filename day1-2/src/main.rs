// Advent of Code — Day 1 Problem 2 Solution

use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "day1-1.input";

fn main() {
  let input = File::open(INPUT_PATH).unwrap();
  let buffered = BufReader::new(input);

  let mut nums: Vec<u32> = Vec::new();

  for line in buffered.lines() {
    nums.push(line.unwrap().parse::<u32>().unwrap())
  }


  for num1 in &nums {
    for num2 in &nums {
      for num3 in &nums {
        if (num1 != num2) && (num2 != num3) && (num1 != num3) && (num1 + num2 + num3 == 2020) {
          println!("{}", num1*num2*num3);
          return;
        }
      }
    }
  }
}
