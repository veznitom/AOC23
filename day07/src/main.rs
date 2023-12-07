use std::fs;

fn parse_input(lines: &Vec<&str>) -> Vec<Vec<(String, i32)>> {
    vec![]
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day07/example").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let sorted = parse_input(&lines);
}
