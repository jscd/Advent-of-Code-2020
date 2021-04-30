use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TV {
    Hash,
    Dot,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Tile {
    vals: [[TV; 10]; 10],
    vals_rot: [[TV; 10]; 10],
    pub id: u128,
}

impl Tile {
    pub fn new(instr: &str) -> Tile {
        let instr = instr.trim();
        let mut lines = instr.split("\n");

        let num_line = lines.next().unwrap();
        let num = num_line[5..num_line.len()-1].parse::<u128>().unwrap();

        let mut vals = [[TV::Dot; 10]; 10];
        let mut vals_rot = [[TV::Dot; 10]; 10];
        for (row, remaining_line) in lines.enumerate() {
            for (col, ch) in remaining_line.chars().enumerate() {
                if let '#' = ch {
                    vals[row][col] = TV::Hash;
                    vals_rot[col][row] = TV::Hash;
                }
            }
        }

        Tile {
            vals: vals,
            vals_rot: vals_rot,
            id: num,
        }
    }

    pub fn has_match(&self, other: &Tile) -> bool {
        let s_sides = [ self.vals[0], self.vals[9], self.vals_rot[0], self.vals_rot[9] ];
        let o_sides = [ other.vals[0], other.vals[9], other.vals_rot[0], other.vals_rot[9] ];

        for s_side in &s_sides {
            let mut s_side_rev = s_side.clone();
            s_side_rev.reverse();

            for o_side in &o_sides {
                if *s_side == *o_side || s_side_rev == *o_side {
                    return true;
                }
            }
        }

        false
    }

}

pub struct Puzzle {
    pub tiles: HashMap<u128, Tile>,
    ordering: Vec<Vec<u128>>,
}


impl Puzzle {
    pub fn new(tiles: HashMap<u128, Tile>) -> Puzzle {
        Puzzle {
            tiles: tiles,
            ordering: Vec::new(),
        }
    }

    pub fn num_matches(&self, tile_id: u128) -> u128 {
        let mut matches = 0;
        let toi = &self.tiles[&tile_id];

        for tile in self.tiles.values() {
            if tile.id == tile_id { continue; }

            if toi.has_match(&tile) { matches += 1; }
        }

        matches
    }

    pub fn get_corners(&self) -> Vec<&Tile> {
        let mut res = Vec::new();

        for tile in self.tiles.values() {
            if self.num_matches(tile.id) == 2 {
                res.push(tile);
            }
        }

        res
    }

    pub fn get_outer(&self, exclusions: Vec<u128>) -> Vec<(&Tile, bool)> {
        let mut res = Vec::new();

        for tile in self.tiles.values() {
            if !exclusions.contains(tile.id) {
                let matches = self.num_matches(tile.id);
                if matches == 3 {
                    res.push((tile, false));
                } else if matches == 2 {
                    res.push((tile, true));
                }
            }
        }

        res
    }

    pub fn arrange(&mut self) {
        let mut exclude = Vec::new();

        
    }
 
}
