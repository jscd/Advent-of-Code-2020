use std::fs;
use std::collections::HashMap;

mod tiles;
use tiles::*;

const INPUT_PATH: &str = "day20.input";

fn main() {
    let puzzle = parse_input(INPUT_PATH);
    let p1 = p1(&puzzle);

    println!("p1: {}", p1);
}

fn parse_input(path: &str) -> Puzzle {
    let input_str = fs::read_to_string(path).unwrap();
    let input_str = input_str.trim();

    let tile_strs = input_str.split("\n\n");

    let mut tiles = HashMap::new();

    for tile_str in tile_strs {
        let cur_tile = Tile::new(tile_str);
        tiles.insert(cur_tile.id, cur_tile);
    }

    Puzzle::new(tiles)
}

fn p1(puzz: &Puzzle) -> u128 {
    let corn = puzz.get_corners();
    corn[0].id * corn[1].id * corn[2].id * corn[3].id
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT_PATH: &str = "day20_test.input";

    #[test]
    fn test_p1() {
        let puzz = parse_input(TEST_INPUT_PATH);
        let p1 = p1(&puzz);

        assert_eq!(p1, 20899048083289); 
    }
    
}
