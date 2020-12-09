use std::fs;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

const INPUT_PATH: &str = "day7.input";

type Vertex = u64;

struct Edge {
    end: Vertex,
    weight: u32,
}

struct Graph {
    vertices: Vec<Vertex>,
    edges: HashMap<Vertex, Vec<Edge>>,
}

impl Graph {
    fn add_vertex(&mut self, vert: Vertex) {
        self.vertices.push(vert);
    }

    fn add_edge(&mut self, start: Vertex, end: Vertex, weight: u32) {
        let new_edge = Edge {
            end,
            weight,
        };
        
        let edge_vec = self.edges.entry(start).or_insert(Vec::new());
        
        edge_vec.push(new_edge);
    }

    fn dfs(&self, start: Vertex) -> Vec<Vertex> {
        let mut visited = Vec::new();
        let mut visit_next = Vec::new();

        visit_next.push(start);
        while !visit_next.is_empty() {
            let cur = visit_next.pop().unwrap();

            if !visited.contains(&cur) {
                visited.push(cur);
                
                match self.edges.get(&cur) {
                    None => continue,
                    Some(edges) => {
                        for edge in edges {
                            visit_next.push(edge.end);
                        }
                    },
                }

            }
        }

        visited
    }

    fn new() -> Graph {
        Graph {
            vertices: Vec::new(),
            edges: HashMap::new(),
        }
    }
}

fn main() {
    let input_str = fs::read_to_string(INPUT_PATH).unwrap();
    let input_str = input_str.trim();
    let (g, g_rev) = parse_data(&input_str);

    let shiny_gold = my_hash("shiny gold") as Vertex;

    // Part 1
    let visitable_from_shiny_gold = g_rev.dfs(shiny_gold);
    println!("P1: {}", visitable_from_shiny_gold.len() - 1);

    // Part 2
    
}


fn parse_data(input: &str) -> (Graph, Graph)  {
    let mut g = Graph::new();
    let mut g_rev = Graph::new();

    for line in input.split("\n") {
        let mut halves = line.split("bags contain");

        let primary_bag_str = halves.next().unwrap().trim();
        let primary_bag = my_hash(primary_bag_str);

        //println!("found {} => {}", primary_bag_str, primary_bag);
        g.add_vertex(primary_bag);
        g_rev.add_vertex(primary_bag);


        let contains_half = halves.next().unwrap();
        for contained_str in contains_half.split(", ") {
            if contained_str.contains("no") { break; }
            let contained_str = contained_str.trim();


            let mut contained_parts = contained_str.split(" ");

            let quantity = contained_parts.next().unwrap().parse::<u32>().unwrap();

            // Discard the "bags," or "bag," or "bags." etc...
            let contained_parts = contained_parts.take(2); 

            let bag_str = contained_parts.fold(String::new(), |acc, next| acc + " " + next);
            let bag_hash = my_hash(&bag_str.trim());

            //println!("\t  -> {}", bag_str);            

            g.add_edge(primary_bag, bag_hash, quantity);
            g_rev.add_edge(bag_hash, primary_bag, quantity);
            
        }
    }

    (g, g_rev)
}

fn my_hash(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);

    hasher.finish()
}
