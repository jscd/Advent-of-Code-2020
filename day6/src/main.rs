
use std::fs;

const INPUT_PATH: &str = "day6.input";
//const INPUT_PATH: &str = "test.input";
const A_NUM: u32 = 'a' as u32;

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let groups = input.split("\n\n");
    let mut sum1 = 0;
    let mut sum2 = 0;

    for group in groups {
        let mut res = make_vec();


        let people = group.trim().split("\n");
        let mut num_ppl = 0;
        for person in people {
            num_ppl += 1;
            for c in person.chars() {
                let num = c as u32;
                res[(num - A_NUM) as usize] += 1;
            }
        }


        //println!("q: {:?}", res);
        //println!("num_ppl: {}", num_ppl);
        for q in res {
            if q >= 1 {
                sum1 += 1;
            }
            if q == num_ppl {
                sum2 += 1;
            }
        }
        //println!("sum2 now: {}", sum2);
    }

    println!("sum1: {}", sum1);
    println!("sum2: {}", sum2);
}

fn make_vec() -> Vec<u32> {
    let mut res = Vec::new();
    for _ in 0..26 {
        res.push(0);
    }

    res
}
