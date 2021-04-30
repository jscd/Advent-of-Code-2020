use std::fs;
use std::collections::HashMap;

const INPUT_PATH: &str = "day15.input";

fn main() {
    let input_str = fs::read_to_string(INPUT_PATH).unwrap();
    let input_str = input_str.trim();

    let nums = parse_input(&input_str);

    let p1 = play_game(&nums, 2020);
    println!("p1: {}", p1);

    let p2 = play_game(&nums, 30000000);
    println!("p2: {}", p2);
}

fn play_game(nums: &Vec<u32>, stop: usize) -> u32 {
    let stop = stop-1;
    let mut nums = nums.clone();
    let mut hist = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        hist.insert(*num, i as u32);
    }

    for i in nums.len()..=stop {
        let new = match val_age(nums[i-1], (i-1) as u32, &mut hist){
            None => 0,
            Some(x) => x,
        };

        //println!("{}", new);
        nums.push(new);
    }


    *nums.last().unwrap()

}

fn val_age(num: u32, cur_pos: u32, hist: &mut HashMap<u32, u32>) -> Option<u32> {
    //println!("hist: {:?}", hist);
    if !hist.contains_key(&num) {
        hist.insert(num, cur_pos);

        //println!("{} has never seen before!", num);
        None
    } else {
        let age = cur_pos - hist[&num];
        //println!("{}\tis {}-{}={} old!", num, cur_pos, hist[&num], age);

        hist.remove(&num);
        hist.insert(num, cur_pos);

        Some(age)
    }
}

fn parse_input(input_str: &str) -> Vec<u32> {
    let mut vec = Vec::new();

    for line in input_str.lines() {
        let num = line.trim().parse::<u32>().unwrap();
        vec.push(num);
    }

    vec
}


#[cfg(test)]
mod test {
    use super::*;

    const TEST1_INPUT_PATH: &str = "day15_test1.input";
    const TEST2_INPUT_PATH: &str = "day15_test2.input";
   
    #[test] 
    fn p1_test_case_1_1(){
        let input_str = fs::read_to_string(TEST1_INPUT_PATH).unwrap();
        let input_str = input_str.trim();


        let nums = parse_input(&input_str);

        assert_eq!(play_game(&nums, 10), 0);
    }

    #[test]
    fn p1_test_case_1_2(){
        let input_str = fs::read_to_string(TEST1_INPUT_PATH).unwrap();
        let input_str = input_str.trim();


        let nums = parse_input(&input_str);

        assert_eq!(play_game(&nums, 2020), 436);
    }

    #[test]
    fn p1_test_case_2(){
        let input_str = fs::read_to_string(TEST2_INPUT_PATH).unwrap();
        let input_str = input_str.trim();

        let nums = parse_input(&input_str);

        assert_eq!(play_game(&nums, 2020), 1836);
    }
}
