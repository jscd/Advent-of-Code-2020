// Advent of Code Day 5 solution

use std::fs;

const INPUT_PATH: &str = "day5.input";

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let partitions = input.split("\n");

    let mut largest = 0;
    
    let mut v = Vec::new();

    for line in partitions {

        let mut row = 0;
        let mut col = 0;
        let mut pow_2row = 64;
        let mut pow_2col = 4;

        for ch in line.chars() {
            if ch == 'F' {
                pow_2row /= 2;
            } else if ch == 'B' {
                row += pow_2row;
                pow_2row /= 2;
            } else if ch == 'L' {
                pow_2col /= 2;
            } else if ch == 'R' {
                col += pow_2col;
                pow_2col /= 2;
            }
        }


        let res = row * 8 + col;
        largest = if res > largest { res } else { largest };

        v.push(res);

    }


    println!("{}", largest);

    
    v.sort();
    let mut last_val = 0;
    for val in v {
        if val - last_val > 1 {
            println!("{} ---- {}", last_val, val);
        }
        last_val = val;
    }
    
}
