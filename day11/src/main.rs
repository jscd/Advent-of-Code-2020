use std::fs;
use std::io::stdin;
use std::cmp;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    Occupied,
    Empty,
}

const INPUT_PATH: &str = "day11.input";
//const INPUT_PATH: &str = "day11_testcase.input";

fn main() {
    let input_str = fs::read_to_string(INPUT_PATH).unwrap();
    let input_str = input_str.trim();


    let seats = parse_input(&input_str);
    print_tiles(&seats);

    let p1 = part1(&seats);
    println!("p1: {}", p1);

    let p2 = part2(&seats);
    println!("p2: {}", p2);
}


fn step(seats: &Vec<Vec<Tile>>, chng_fn: &dyn Fn(usize, usize, &Vec<Vec<Tile>>) -> Tile) -> Vec<Vec<Tile>> {
    let mut res = Vec::new();

    for row_num in 0..seats.len() {
        res.push(Vec::new());
        for col_num in 0..seats[row_num].len() {
            match seats[row_num][col_num] {
                Tile::Floor => res[row_num].push(Tile::Floor),
                Tile::Occupied => res[row_num].push(chng_fn(row_num, col_num, &seats)),
                Tile::Empty => res[row_num].push(chng_fn(row_num, col_num, &seats)),
            }
        }
    }

    res
}

fn part1(seats: &Vec<Vec<Tile>>) -> u32 {
    let mut seats = seats.clone();
    let mut next = step(&seats, &adjacent_cmp);

    let height = seats.len();
    let width = seats[0].len();
    println!("dims: {}x{}", width, height);

    let mut generations = 0;

    while !seats_equal(&seats, &next) {
        generations += 1; 

        /*
        println!("===============================================");
        println!("gen: {}" , generations);
        print_tiles(&seats);
        println!("-----------------------------------------------");
        print_tiles(&next);
        let mut s = String::new();
        stdin().read_line(&mut s);
        */

        seats = next;
        next = step(&seats, &adjacent_cmp);
    }

    println!("took {} generations", generations);
    
    count_occupied(&next)
}

fn part2(seats: &Vec<Vec<Tile>>) -> u32 {
    let mut seats = seats.clone();
    let mut next = step(&seats, &visible_cmp);

    let mut generations = 0;
    
    while !seats_equal(&seats, &next) {
        generations += 1;

        /*
        println!("===============================================");
        println!("gen: {}" , generations);
        print_tiles(&seats);
        println!("-----------------------------------------------");
        print_tiles(&next);
        let mut s = String::new();
        stdin().read_line(&mut s);
        */

        seats = next;
        next = step(&seats, &visible_cmp);
    }

    println!("took {} generations", generations);
    
    count_occupied(&next)
}

fn count_occupied(seats: &Vec<Vec<Tile>>) -> u32 {
    let mut num_occ = 0;
    for row in seats {
        for col in row {
            if let Tile::Occupied = col { num_occ += 1; }
        }
    }

    num_occ
}

fn visible_cmp(row: usize, col: usize, seats: &Vec<Vec<Tile>>) -> Tile {
    match seats[row][col] {
        Tile::Floor => Tile::Floor,
        Tile::Occupied => {
            let vis = get_visible_occupied(row, col, &seats);
            //println!("[{}, {}] occupied w/ {} vis", row, col, vis);
            if vis >= 5 {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        },
        Tile::Empty => {
            let vis = get_visible_occupied(row, col, &seats);
            if vis == 0 {
                Tile::Occupied
            } else {
                Tile::Empty
            }
        }
    }
}

fn adjacent_cmp(row: usize, col: usize, seats: &Vec<Vec<Tile>>) -> Tile {
    match seats[row][col] {
        Tile::Floor => Tile::Floor,
        Tile::Occupied => {
            let adj = get_adjacent_occupied(row, col, &seats);
            //println!("[{},{}] got occupied w/ {} adj", row, col, adj);
            if adj >= 4 {
                //println!("changing to empty...");
                Tile::Empty
            } else {
                Tile::Occupied
            }
        },
        Tile::Empty => {
            let adj = get_adjacent_occupied(row, col, &seats);
            //println!("[{},{}] got empty w/ {} adj", row, col, adj);
            if adj == 0 {
                //println!("changing to occupied...");
                Tile::Occupied
            } else {
                Tile::Empty
            }
        }
    }
}

fn seats_equal(a: &Vec<Vec<Tile>>, b: &Vec<Vec<Tile>>) -> bool {
    if a.len() != b.len() { return false; }
    if a[0].len() != b[0].len() { return false; }

    let height = a.len();
    let width = a[0].len();

    for row in 0..height {
        for col in 0..width {
            if a[row][col] != b[row][col] { return false; }
        }
    }

    true
}

fn get_visible_occupied(row: usize, col: usize, seats: &Vec<Vec<Tile>>) -> u32 {
    // There are 8 lines to be drawn from the center
    let r  = get_slope_occupied(row, col,  1,  0, seats);
    let l  = get_slope_occupied(row, col, -1,  0, seats);
    let d  = get_slope_occupied(row, col,  0,  1, seats);
    let u  = get_slope_occupied(row, col,  0, -1, seats);
    let ur = get_slope_occupied(row, col,  1, -1, seats);
    let dr = get_slope_occupied(row, col,  1,  1, seats);
    let dl = get_slope_occupied(row, col, -1,  1, seats);
    let ul = get_slope_occupied(row, col, -1, -1, seats);

    r + l + d + u + ur + dr + dl + ul
}

fn get_slope_occupied(rstart: usize, cstart: usize, xinc: i32, yinc: i32, seats: &Vec<Vec<Tile>>) -> u32 {
    let mut r = (rstart as i32) + yinc;
    let mut c = (cstart as i32) + xinc;

    while r >= 0 && r < (seats.len() as i32) &&
          c >= 0 && c < (seats[0].len() as i32) {

        match seats[r as usize][c as usize] {
            Tile::Floor => {
                r += yinc;
                c += xinc;
            },
            Tile::Occupied => return 1,
            Tile::Empty => return 0,
        }
    }

    0
}

fn get_adjacent_occupied(row: usize, col: usize, seats: &Vec<Vec<Tile>>) -> u32 {
    let mut adj_occ = 0;

    let lower_row = cmp::max(row as i32 - 1, 0) as usize;
    let upper_row = cmp::min(row+1, seats.len()-1);
    let lower_col = cmp::max(col as i32 -1, 0) as usize;
    let upper_col = cmp::min(col+1, seats[0].len()-1);
    //println!("row: [{} - {}] -- col: [{} - {}]", lower_row, upper_row, lower_col, upper_col);

    for r in lower_row..=upper_row {
        for c in lower_col..=upper_col {
            if let Tile::Occupied = seats[r][c] {
                if r != row || c != col { adj_occ += 1; }
            }
        }
    }

    adj_occ
}


fn print_tiles(v: &Vec<Vec<Tile>>) {
    for row in v {
        for col in row {
            match col {
                Tile::Empty => print!("L"),
                Tile::Occupied => print!("#"),
                Tile::Floor => print!("."),
            }
        }
        println!();
    }
}

fn parse_input(input_str: &str) -> Vec<Vec<Tile>> {
    let mut v = Vec::new();

    for line in input_str.lines() {
        let mut row = Vec::new();
        for chr in line.chars() {
            match chr {
                '.' => row.push(Tile::Floor),
                '#' => row.push(Tile::Occupied),
                'L' => row.push(Tile::Empty),
                 _  => (),
            }
        }
        v.push(row);
    }

    v
}

