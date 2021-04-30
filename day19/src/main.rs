use std::fs;
use std::collections::HashMap;

const INPUT_PATH: &str = "day19.input";

struct Vertex {
    id: u32,
    chars: Vec<char>,
}

struct Graph {
    vertices: Vec<Vertex>,
    edges: HashMap<Vertex, Vec<Vertex>>,
}

impl Graph {
    fn add_vertex(&mut self, vert: Vertex) {
        self.vertices.push(vert);
    }

    fn add_edge(&mut self, start: Vertex, end: Vertex) {
        let edge_vec = self.edges.entry(start).or_insert(Vec::new());
        edge_vec.push(end);
    }

    fn new() -> Graph {
        Graph {
            vertices: Vec::new(),
            edges: HashMap::new(),
        }
    }
}

struct Ruleset {
    g: Graph,
}

impl Ruleset {
    fn new() -> Ruleset {
        Ruleset {
            g: Graph::new(),
        }
    }

    fn add_rules(&mut self, rules_str: &str) {
        let mut id = 0;
        for rline in rules_str.split("\n") {
            let rline = rline.trim();

            
            
            id += 1;
        }
    }
}

fn main() {
    
}

fn parse_input(path: &str) -> OUTPUT {
    let input_str = fs::read_to_string(path).unwrap();
    let input_str = input_str.trim();

    let input_str_split = input_str.spli("\n\n");

    let rules_str = input_str_split[0];
    let messages_str = input_str_split[1];

}


#[cfg(test)]
mod tests {
    const TEST_INPUT_PATH: &str = "day19_test.input";

    #[test]
    fn test_p1() {
        let (rules, messages) = parse_input(TEST_INPUT_PATH);

        let num_r0 = get_matches(rules, messages, 0);

        assert_eq!(num_r0, 2);
    }
}
