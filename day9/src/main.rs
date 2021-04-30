use std::fs;

const INPUT_PATH: &str = "day9.input";


fn main() {
    let input_str = fs::read_to_string(INPUT_PATH).unwrap();
    let input_str = input_str.trim();

    let input = parse_input(&input_str);
    
    // Part 1
    let p1 = part1(&input);
    println!("p1: {}", p1);

    // Part 2
    let p2 = part2(&input, p1);
    println!("p2: {}", p2);
}

fn part1(v: &Vec<u64>) -> u64 {
    for i in 25..v.len() {
        let options = &v[i-25..i];
        let cur_num = v[i];

        if !get_if_sum(cur_num, &options) {
            return cur_num;
        }
    }

    0
}

fn part2(v: &Vec<u64>, goal: u64) -> u64 {
    for size in 2..v.len() {
        for i in 0..(v.len() - size) {
            let slice = &v[i..i+size];
            let sum: u64 = slice.iter().sum();

            if sum == goal {
                let mut r = Vec::new();
                r.extend_from_slice(&v[i..i+size]);

                r.sort();

                let mut r = r.iter();
                
                let small = r.next().unwrap();
                let big = r.last().unwrap();

                return big + small;
            }
        }
    }

    0
}

fn get_if_sum(num: u64, options: &[u64]) -> bool {
    for o1 in options {
        for o2 in options {
            if o1 + o2 == num { return true; }
        }
    }

    false
}

fn parse_input(input: &str) -> Vec<u64> {
    let mut res = Vec::new();

    for line in input.split("\n") {
        let num = line.parse::<u64>().unwrap();
        res.push(num);
    }

    res
}
