// Advent of Code — Day 3 Problem 1 Solution

use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "day3.input";

fn main() {
  let input = File::open(INPUT_PATH).unwrap();
  let input = BufReader::new(input);

  let map = get_trees(input);

  let p1_sol = trees_with_slope(3, -1, &map);
  println!("p1: {}", p1_sol);


  let s11 = trees_with_slope(1, -1, &map);
  let s31 = p1_sol;
  let s51 = trees_with_slope(5, -1, &map);
  let s71 = trees_with_slope(7, -1, &map);
  let s12 = trees_with_slope(1, -2, &map);

  let p2_sol = s11 * s31 * s51 * s71 * s12;


  println!("p2: {}", p2_sol);
}

fn trees_with_slope(x_slope: i32, y_slope: i32, map: &Vec<Vec<bool>>) -> u32 {
  let mut num_trees = 0;

  let mut x: usize = 0;
  let mut y: usize = 0;

  let map_height = map.len();
  let map_width = map[0].len();

  while y < map_height {
    if map[y][x] {
      num_trees += 1;
    }

    x += x_slope as usize;
    y += (-y_slope) as usize; // bc going down is going up when measuring from top left

    if x >= map_width {
      x -= map_width;
    }
  }

  num_trees
}

fn get_trees(buf_input: BufReader<File>) -> Vec<Vec<bool>> {
  let mut map: Vec<Vec<bool>> = Vec::new();

  for line in buf_input.lines() {
    let line = line.unwrap();
    let mut line_trees = Vec::new();

    for c in line.chars() {
      line_trees.push(
        if c == '.' { false }
        else { true }
      );
    }

    map.push(line_trees);
  }

  map
}
