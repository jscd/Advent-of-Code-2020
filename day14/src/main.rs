use std::fs;

const INPUT_PATH: &str = "day14.input";

#[derive(Clone, Copy)]
struct u36 {
    raw: u64,
}

#[derive(Clone, Copy)]
enum MaskVal {
    X,
    One,
    Zero,
}


impl u36 {
    fn zero() -> u36 {
        u36 { raw: 0 }
    }

    fn new(i: u64) -> u36 {
        u36 {
            raw: i % 2.pow(36)
        }
    }

    fn get(&self) -> u64 {
        self.raw % 2.pow(36)
    }

    fn mask(&self, mask: Vec<MaskVal>) -> u36 {
        let mut new = u36::zero();
        
        for i in 0..36 {
            if let MaskVal::One = mask[i] {
                new += 2.pow(i);
            } else if let MaskVal::X = mask[i] {
                new += (2.pow(i)) & (self.raw);
            }
        }

        new
    }
}

fn main() {
    let input_str = fs::read_to_string(INPUT_PATH).unwrap();
    let input_str = input_str.trim();
}

fn parse_input(input_str: &str) -> (Vec<i8>, Vec<(u32, u36)>) {
    let mut mask = Vec::new();
    let mut instrs = Vec::new();

    let mut lines = input_str.lines();
    
    for chr in lines.next().unwrap().chars() {
        match chr {
            'X' => mask.push(MaskVal::X),
            '1' => mask.push(MaskVal::One),
            '0' => mask.push(MaskVal::Zero),
            _ => (),
        }
    }

    for line in lines {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_PATH = "day14_testcase.input";

    fn test_parse() {
        let input_str = fs::read_to_string(TEST_INPUT_PATH).unwrap();
        let input_str = input_str.trim();

        let parsed = parse_input(&input_str);
    }
}


