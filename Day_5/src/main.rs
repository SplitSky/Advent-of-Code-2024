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

    fn fix_order(&self, update: &mut Vec<i32>) -> bool {
        // Create a position lookup table for the elements in `update`
        let mut position: HashMap<i32, usize> = update
            .iter()
            .enumerate()
            .map(|(index, &page)| (page, index))
            .collect();

        // Check for rule violations and swap elements
        for (&from, targets) in &self.adj_list {
            if let Some(&from_pos) = position.get(&from) {
                for &to in targets {
                    if let Some(&to_pos) = position.get(&to) {
                        if from_pos >= to_pos {
                            // Swap the elements in the update vector
                            update.swap(from_pos, to_pos);
                            // Update the lookup table since indices have changed
                            position.insert(update[from_pos], from_pos);
                            position.insert(update[to_pos], to_pos);
                        }
                    }
                }
            }
        }
        return true;
    }

    fn fix_order_2(&self, update: &mut Vec<i32>) -> bool {
        use std::collections::{HashMap, VecDeque};

        let mut in_degree: HashMap<i32, usize> = HashMap::new();
        for &mut item in &mut *update {
            in_degree.entry(item).or_insert(0);
        }
        for (&from, targets) in &self.adj_list {
            if in_degree.contains_key(&from) {
                for &to in targets {
                    if in_degree.contains_key(&to) {
                        *in_degree.entry(to).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut queue: VecDeque<i32> = VecDeque::new();
        for (&node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node);
            }
        }

        let mut sorted_order: Vec<i32> = Vec::new();
        while let Some(node) = queue.pop_front() {
            sorted_order.push(node);
            if let Some(targets) = self.adj_list.get(&node) {
                for &neighbor in targets {
                    if let Some(in_deg) = in_degree.get_mut(&neighbor) {
                        *in_deg -= 1;
                        if *in_deg == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        if sorted_order.len() != in_degree.len() {
            return false;
        }

        let position_map: HashMap<i32, usize> = sorted_order
            .iter()
            .enumerate()
            .map(|(i, &value)| (value, i))
            .collect();
        update.sort_by_key(|value| position_map.get(value).cloned().unwrap_or(usize::MAX));

        true
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

    total = 0;
    // fix the incorrect ones and add them up
    for update in wrong_list.iter_mut() {
        // fix the order
        graph.fix_order_2(update);
    }
    // count of the correct instruction middle values
    for update in wrong_list {
        if graph.is_order_valid(&update) == true {
            total += extract_middle(&update);
        }
    }
    println!("total {}", total);
    Ok(())
}
