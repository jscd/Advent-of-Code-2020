use std::fs;

const INPUT_PATH: &str = "day12.input";

#[derive(Clone, Copy)]
enum Dir {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

#[derive(Clone, Copy)]
enum Movement {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}


#[derive(Clone, Copy)]
struct Position {
    vertical: i32, // -north/+south
    horizontal: i32, // -west/+east
    dir: Dir,
}

impl Position {
    fn new() -> Position {
        Position {vertical: 0, horizontal: 0, dir: Dir::East}
    }
    
    fn move_point(&mut self, mv: &Movement) {
        match mv {
            Movement::North(d) => self.vertical -= d,
            Movement::South(d) => self.vertical += d,
            Movement::East(d) => self.horizontal += d,
            Movement::West(d) => self.horizontal -= d,
            
            Movement::Left(deg) => {
                let rot = deg/90;
                let new_dir = ((self.dir as i32) + rot) % 4;
                self.dir = match new_dir {
                    0 => Dir::East,
                    1 => Dir::North,
                    2 => Dir::West,
                    3 => Dir::South,
                    _ => panic!("Invalid direction!")
                };
            },

            Movement::Right(deg) => {
                let rot = deg/90;
                let mut new_dir = ((self.dir as i32) - rot) % 4;
                while new_dir < 0 { new_dir += 4; }

                self.dir = match new_dir {
                    0 => Dir::East,
                    1 => Dir::North,
                    2 => Dir::West,
                    3 => Dir::South,
                    _ => panic!("Invalid direction!")
                };
            },
            Movement::Forward(d) => {
                match self.dir {
                    Dir::East => self.horizontal += d,
                    Dir::West => self.horizontal -= d,
                    Dir::South => self.vertical += d,
                    Dir::North => self.vertical -= d,
                };
            }
        }
    }

    fn move_waypoint(&mut self, mv: &Movement) {
        match mv {
            Movement::North(d) => self.vertical -= d,
            Movement::South(d) => self.vertical += d,
            Movement::East(d) => self.horizontal += d,
            Movement::West(d) => self.horizontal -= d,

            // Left and Right need to rotate point around (0,0)
            Movement::Left(deg) => {
                match deg {
                    0 => (),
                    90 => {
                        let old_vert = self.vertical;
                        let old_hori = self.horizontal;
            
                        self.vertical = -old_hori;
                        self.horizontal = old_vert;
                    },
                    180 => {
                        self.vertical = -self.vertical;
                        self.horizontal = -self.horizontal;
                    },
                    270 => {
                        let old_vert = self.vertical;
                        let old_hori = self.horizontal;

                        self.vertical = old_hori;
                        self.horizontal = -old_vert;
                    },
                    360 => (),
                    _ => panic!("Invalid number of degrees!")
                }
            },
            Movement::Right(deg) => {
                match deg {
                    0 => (),
                    90 => {
                        let old_vert = self.vertical;
                        let old_hori = self.horizontal;
            
                        self.vertical = old_hori;
                        self.horizontal = -old_vert;
                    },
                    180 => {
                        self.vertical = -self.vertical;
                        self.horizontal = -self.horizontal;
                    },
                    270 => {
                        let old_vert = self.vertical;
                        let old_hori = self.horizontal;

                        self.vertical = -old_hori;
                        self.horizontal = old_vert;
                    },
                    360 => (),
                    _ => panic!("Invalid number of degrees!")
                }
            },
            Movement::Forward(_) => panic!("Don't give Forward to waypoint!"),
        }
    }

    fn get_mdist(&self) -> i32 {
        let v = if self.vertical < 0 { -self.vertical } else { self.vertical };
        let h = if self.horizontal < 0 { -self.horizontal } else { self.horizontal };

        v + h
    }
}

fn go_through(start: &mut Position, moves: &Vec<Movement>) -> i32 {
    for movement in moves {
        start.move_point(movement);
    }

    start.get_mdist()
}

fn go_waypoint(waypoint: &mut Position, moves: &Vec<Movement>) -> i32 {
    let mut ship_pos = Position::new();

    for movement in moves {
        if let Movement::Forward(d) = movement {
            ship_pos.vertical += d*waypoint.vertical;
            ship_pos.horizontal += d*waypoint.horizontal;
        } else {
            waypoint.move_waypoint(movement);
        }
    }

    ship_pos.get_mdist()
}

fn parse_input(input_str: &str) -> Vec<Movement>  {
    let mut mvmnt = Vec::new();

    for line in input_str.lines() {
        let mut chars = line.trim().chars();
        
        let instr = chars.next().unwrap();
        let val = chars.collect::<String>().parse::<i32>().unwrap();

        let mv = match instr {
            'F' => Movement::Forward(val),
            'N' => Movement::North(val),
            'S' => Movement::South(val),
            'E' => Movement::East(val),
            'W' => Movement::West(val),
            'L' => Movement::Left(val),
            'R' => Movement::Right(val),
            _ => panic!("Invalid movement!"),
        };

        mvmnt.push(mv);
    }

    mvmnt
}

fn main() {
    let input_str = fs::read_to_string(INPUT_PATH).unwrap();
    let input_str = input_str.trim();

    let input = parse_input(&input_str);

    // Part 1
    let mut position = Position::new();

    let p1 = go_through(&mut position, &input);
    println!("p1: {}", p1);

    let mut waypoint = Position { vertical: -1, horizontal: 10, dir: Dir::East};
    let p2 = go_waypoint(&mut waypoint, &input);
    println!("p2: {}", p2);
}


// ==================================================

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_PATH: &str = "day12_testcase.input";

    #[test]
    fn test_part1() {
        let input_str = fs::read_to_string(TEST_INPUT_PATH).unwrap();
        let input_str = input_str.trim();

        let parsed = parse_input(&input_str);
        
        // test part1
        let mut pos = Position::new();
        let mdist = go_through(&mut pos, &parsed);

        assert_eq!(mdist, 25);

    }

    #[test]
    fn test_part2() {
        let input_str = fs::read_to_string(TEST_INPUT_PATH).unwrap();
        let input_str = input_str.trim();

        let parsed = parse_input(&input_str);
        
        // test part2
        let mut waypoint = Position { vertical: -1, horizontal: 10, dir: Dir::East };
        let dist = go_waypoint(&mut waypoint, &parsed);

        assert_eq!(dist, 286);
    }
}
