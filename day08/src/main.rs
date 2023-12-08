use std::{
    collections::{HashMap, HashSet},
    fs
};

use num::integer::lcm;

fn pritn_directions(directions: &HashMap<String, (String, String)>) {
    for dir in directions {
        println!("{} -> ({}, {})", dir.0, dir.1 .0, dir.1 .1);
    }
}

fn parse_input(lines: &Vec<&str>) -> (String, HashMap<String, (String, String)>) {
    let guide: String = lines[0].to_string();
    let mut directions: HashMap<String, (String, String)> = HashMap::new();
    for i in 2..lines.len() {
        let strip = lines[i].replace(" ", "").replace("(", "").replace(")", "");
        let split: Vec<&str> = strip.split("=").collect();
        let right: Vec<&str> = split[1].split(",").collect();
        directions.insert(
            split[0].to_string(),
            (right[0].to_string(), right[1].to_string()),
        );
    }
    (guide, directions)
}

fn follow_guide(start: &str, guide: &str, directions: &HashMap<String, (String, String)>) -> i64 {
    let mut current = start;
    let mut path: i64 = 0;
    for c in guide.chars() {
        if current == "ZZZ" {
            return path;
        }
        if c == 'R' {
            current = directions.get(current).unwrap().1.as_str();
        } else {
            current = directions.get(current).unwrap().0.as_str();
        }
        path += 1;
    }
    path + follow_guide(current, guide, directions)
}

fn follow_ghost_guide(
    start: &str,
    guide: &str,
    directions: &HashMap<String, (String, String)>,
) -> i64 {
    let mut current = start;
    let mut path: i64 = 0;
    for c in guide.chars() {
        if current.ends_with("Z") {
            return path;
        }
        if c == 'R' {
            current = directions.get(current).unwrap().1.as_str();
        } else {
            current = directions.get(current).unwrap().0.as_str();
        }
        path += 1;
    }
    path + follow_ghost_guide(current, guide, directions)
}

fn get_reachable(
    states: &HashSet<String>,
    directions: &HashMap<String, (String, String)>,
) -> HashSet<String> {
    let mut reachable: HashSet<String> = HashSet::new();
    for state in states {
        if state.ends_with("A") {
            reachable.insert(state.clone());
        }
    }
    let mut last_size: usize = 0;
    while reachable.len() != last_size {
        last_size = reachable.len();
        for r in reachable.clone() {
            reachable.insert(directions.get(&r).unwrap().0.clone());
            reachable.insert(directions.get(&r).unwrap().1.clone());
        }
    }
    reachable
}

fn get_useful(
    states: &HashSet<String>,
    directions: &HashMap<String, (String, String)>,
) -> HashSet<String> {
    let mut useful: HashSet<String> = HashSet::new();
    for state in directions {
        if state.1 .0.ends_with("Z") || state.1 .1.ends_with("Z") {
            useful.insert(state.0.clone());
        }
    }
    let mut last_size: usize = 0;
    while useful.len() != last_size {
        last_size = useful.len();
        for d in directions {
            if useful.contains(&d.1 .0) || useful.contains(&d.1 .1) {
                useful.insert(d.0.clone());
            }
        }
    }
    useful
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day08/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let (guide, directions) = parse_input(&lines);
    let states: HashSet<String> = HashSet::from_iter(directions.keys().cloned());
    let reachable = get_reachable(&states, &directions);
    let useful = get_useful(&states, &directions);
    println!(
        "All: {}, Reachable: {}, Useful: {}",
        states.len(),
        reachable.len(),
        useful.len()
    );
    println!("{}", guide);
    pritn_directions(&directions);
    println!("Walked: {}", follow_guide("AAA", &guide, &directions));
    let xxa: Vec<i64> = directions
        .keys()
        .into_iter()
        .filter(|x| x.ends_with("A"))
        .cloned()
        .map(|x| follow_ghost_guide(x.as_str(), &guide, &directions))
        .collect();
    println!("Fuck you: {}", xxa.into_iter().fold(1, |a,b| lcm(a,b)))
}
