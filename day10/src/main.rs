use std::fs;

const INPUT_PATH: &str = "day10.input";
//const INPUT_PATH: &str = "smalltest.input";
//const INPUT_PATH: &str = "testcase.input";

type Jolt = u32;

fn main() {
    let input_str = fs::read_to_string(INPUT_PATH).unwrap();
    let input_str = input_str.trim();

    let mut jolts = parse_input(&input_str);
    jolts.sort();

    // Part 1
    let mut prev_jolt = 0;
    let mut num3 = 1; // start at one to include the final +3
    let mut num1 = 0;
    for jolt in &jolts {
        let diff = jolt - prev_jolt;

        match diff {
            0 => (),
            1 => num1 += 1,
            2 => (),
            3 => num3 += 1,
            _ => println!("somethin wrong"),
        }

        prev_jolt = *jolt;
    }

    println!("1: {}, 3: {}\np1:{}", num1, num3, num1*num3);

    // Part 2
    let p2 = part2(&jolts);
    println!("p2: {}", p2);
}

fn part2(jolts: &Vec<Jolt>) -> u128 {
    let final_goal = jolts.iter().max().unwrap();

    // # of ways to get to final is the same as # ways to get to final - 3, bc the problem specifies that final is by definition 3 greater than max val
    let init = 0;

    // func(n) is the number of ways to get to n jolts
    let mut func = Vec::new();
    let mut cur: usize = 1;
    func.push(1);

    while (cur as Jolt) <= *final_goal {
        if !jolts.contains(&(cur as Jolt)) {
            func.push(0);
        } else { 
            let val = match cur {
                1 => func[cur-1],
                2 => func[cur-1] + func[cur-2],
                _ => func[cur-1] + func[cur-2] + func[cur-3],
            };

            println!("there are {} ways to reach {}", val, cur);

            func.push(val);
        }

        cur += 1;
    }

    func[cur-1]
}



fn parse_input(input_str: &str) -> Vec<Jolt> {
    let mut res = Vec::new();

    for line in input_str.lines() {
        let num = line.parse::<u32>().unwrap();
        res.push(num);
    }

    res
}
