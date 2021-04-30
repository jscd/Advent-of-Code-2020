use std::fs;

mod allergens;
use allergens::*;

const INPUT_PATH: &str = "day21.input";

fn main() {
    let parsed = parse_input(INPUT_PATH);
}

fn parse_input(path: &str) -> OUTPUT {
    let in_str = fs::read_to_string(path).unwrap();
    let in_str = in_str.trim();

    // parse
    for line in in_str.split("\n") {
        let line = line.trim();
        let 
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_PATH: &str = "day21_test.input";

    #[test]
    fn test_p1() {
        let parsed = parse_input(TEST_INPUT_PATH);

        
    }
}
