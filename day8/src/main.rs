use std::fs;

const INPUT_PATH: &str = "day8.input";

#[derive(Debug, Clone, Copy)]
enum Instr {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn main() {
    let input_str = fs::read_to_string(INPUT_PATH).unwrap();
    let input_str = input_str.trim();

    let mut instructions = parse_input(&input_str);
    println!("got {} instructions", instructions.len());

    
    // Part 1
    println!("p1: {}", part1(&instructions).1);

    // Part 2
    println!("p2: {}", part2(&mut instructions).unwrap());
}

// Does the same this as part 1, but also keeps
// track of which nops and which jmps were ran
// last. After this, swaps those out in FILO or-
// -der until the program terminates normally.
fn part2(instrs: &mut Vec<Instr>) -> Option<i64> {
    let mut accum = 0;
    let mut ip: i64 = 0;
    
    let mut lines_executed = Vec::new();

    let mut jmps_or_nops = Vec::new();

    while (ip as usize) < instrs.len() {
        if lines_executed.contains(&ip) { break; }        

        let cur_instr = &instrs[ip as usize];
        lines_executed.push(ip);

        match cur_instr {
            Instr::Nop(_) => {
                jmps_or_nops.push(ip as usize);
                ip += 1;
            },
            Instr::Acc(i) => { 
                accum += i;
                ip += 1;
            },
            Instr::Jmp(i) => {
                jmps_or_nops.push(ip as usize);
                ip += i;
            }
        }
    }
   
    let mut count = 0; 
    // Tha cool stuff
    while !jmps_or_nops.is_empty() {
        let idx = jmps_or_nops.pop().unwrap();
        count += 1;

        match instrs[idx] {
            Instr::Nop(val) => instrs[idx] = Instr::Jmp(val),
            Instr::Jmp(val) => instrs[idx] = Instr::Nop(val),
            _ => (),
        }

        let (no_loop, accum) = part1(&instrs);
        if no_loop {
            println!("got part 2 solution in {} cycles", count);
            return Some(accum);
        }
        
        match instrs[idx] {
            Instr::Nop(val) => instrs[idx] = Instr::Jmp(val),
            Instr::Jmp(val) => instrs[idx] = Instr::Nop(val),
            _ => (),
        }

    }
    
    None
}

// Run each instruction, stopping when there's 
// an instruction run twice or it reaches the end.
// Returns a tuple containing a bool indicating 
// `true` if it did not have any instruction run
// twice and `false` if otherwise. The second item
// in the tuple is the accumulator when it the
// program was stopped.
fn part1(instrs: &Vec<Instr>) -> (bool, i64) {
    let mut accum = 0;
    let mut ip: i64 = 0;

    let mut lines_executed = Vec::new();

    while (ip as usize) < instrs.len() {
        if lines_executed.contains(&ip) { return (false, accum); }        

        let cur_instr = &instrs[ip as usize];
        lines_executed.push(ip);


        match cur_instr {
            Instr::Nop(_) => ip += 1,
            Instr::Acc(i) => { 
                accum += i;
                ip += 1;
            },
            Instr::Jmp(i) => ip += i,
        }
    }

    (true, accum)
}

// Parse input into a list of instructions
fn parse_input(input: &str) -> Vec<Instr> {
    let mut res = Vec::new();
    
    for line in input.split("\n") {
        let mut parts = line.split(" ");

        let instr = parts.next().unwrap().trim();

        let val = parts.next().unwrap().trim().parse::<i64>().unwrap();

        if instr.contains("nop") {
            res.push(Instr::Nop(val));
        } else if instr.contains("acc") {
            res.push(Instr::Acc(val));
        } else if instr.contains("jmp") {
            res.push(Instr::Jmp(val));
        }

    }
    res
}
