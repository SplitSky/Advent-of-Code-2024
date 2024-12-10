use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Graph {
    adj_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }
    fn add_edge(&mut self, from: i32, to: i32) {
        return self.adj_list.entry(from).or_insert_with(Vec::new).push(to);
    }

    fn is_order_valid(&self, update: &Vec<i32>) -> bool {
        let position: HashMap<i32, usize> = update
            .iter()
            .enumerate()
            .map(|(index, &page)| (page, index))
            .collect();
        // this makes a lookup table of the position of the update along with its value for
        // iterating
        for (&from, targets) in &self.adj_list {
            if let Some(&from_pos) = position.get(&from) {
                for &to in targets {
                    if let Some(&to_pos) = position.get(&to) {
                        if from_pos >= to_pos {
                            return false;
                        }
                    }
                }
            }
        }
        return true;
    }

    fn fix_order(&self, update: &Vec<i32>) -> Vec<i32> {
        // iterate over the updates until rule is violated
        // swap the current tile with the
    }
}

fn extract_middle(update: &Vec<i32>) -> i32 {
    return update[update.len() / 2];
}

fn parse_tuple(line: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            return Some((a, b));
        }
    }
    return None;
}

fn main() -> io::Result<()> {
    let mut path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut reader = io::BufReader::new(file);

    let tuples: Vec<(i32, i32)> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| parse_tuple(&line))
        .collect();
    // populate the graph with tuples
    let mut graph: Graph = Graph::new();
    for (from, to) in &tuples {
        graph.add_edge(*from, *to);
    }

    // get the lists of updates
    path = Path::new("input_updates.txt");
    file = File::open(&path)?;
    reader = io::BufReader::new(file);

    let mut updates: Vec<Vec<i32>> = Vec::new();
    // Process each line
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<i32> = line
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();
        // get updates
        updates.push(parts);
    }
    let mut total = 0;
    let mut wrong_list: Vec<Vec<i32>> = Vec::new();
    for update in updates {
        // check if path is okay
        if graph.is_order_valid(&update) == true {
            total += extract_middle(&update);
        } else {
            wrong_list.push(update);
        }
    }
    println!("total {}", total);

    // fix the incorrect ones and add them up
    for update in wrong_list {
        // fix update
        // get middle
    }

    Ok(())
}
