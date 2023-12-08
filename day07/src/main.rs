use std::{cmp::Ordering, collections::HashSet, fs};

fn print_sorted(sorted: &Vec<Vec<(String, i64)>>) {
    println!("High card");
    for hand in sorted[0].iter() {
        println!("  {}", hand.0);
    }
    println!("One pair");
    for hand in sorted[1].iter() {
        println!("  {}", hand.0);
    }
    println!("Two pair");
    for hand in sorted[2].iter() {
        println!("  {}", hand.0);
    }
    println!("Three of a kind");
    for hand in sorted[3].iter() {
        println!("  {}", hand.0);
    }
    println!("Full house");
    for hand in sorted[4].iter() {
        println!("  {}", hand.0);
    }
    println!("Four of a kind");
    for hand in sorted[5].iter() {
        println!("  {}", hand.0);
    }
    println!("Five of a kind");
    for hand in sorted[6].iter() {
        println!("  {}", hand.0);
    }
}

fn sum_ranked(sorted: &Vec<Vec<(String, i64)>>) -> i64 {
    let mut rank = 1;
    let mut winnings = 0;
    for kind in sorted {
        for hand in kind.iter() {
            winnings += hand.1 * rank;
            rank += 1;
        }
    }
    winnings
}

fn hand_cmp(a: &Vec<char>, b: &Vec<char>) -> Ordering {
    for i in 0..5 {
        match (a[i], b[i]) {
            (x, y) if x == y => continue,
            (x, y) if x.is_alphabetic() && y.is_digit(10) => return Ordering::Greater,
            (x, y) if x.is_digit(10) && y.is_alphabetic() => return Ordering::Less,
            (x, y) if x.is_digit(10) && y.is_digit(10) => match (x, y) {
                (k, l) if k < l => return Ordering::Less,
                (k, l) if k > l => return Ordering::Greater,
                (_, _) => {
                    println!("-------------------------------------------");
                    return Ordering::Equal;
                }
            },
            (x, y) if x.is_alphabetic() && y.is_alphabetic() => match (x, y) {
                (k, l) if k == l => return Ordering::Equal,
                ('A', _) => return Ordering::Greater,
                ('K', 'A') => return Ordering::Less,
                ('K', 'Q') => return Ordering::Greater,
                ('K', 'J') => return Ordering::Greater,
                ('K', 'T') => return Ordering::Greater,
                ('Q', 'A') => return Ordering::Less,
                ('Q', 'K') => return Ordering::Less,
                ('Q', 'J') => return Ordering::Greater,
                ('Q', 'T') => return Ordering::Greater,
                ('J', 'A') => return Ordering::Less,
                ('J', 'K') => return Ordering::Less,
                ('J', 'Q') => return Ordering::Less,
                ('J', 'T') => return Ordering::Greater,
                ('T', _) => return Ordering::Less,
                (_, _) => {
                    println!("-------------------------------------------");
                    return Ordering::Equal;
                }
            },
            (_, _) => {
                println!("-------------------------------------------");
                return Ordering::Equal;
            }
        }
    }
    Ordering::Equal
}

fn hand_cmp_2(a: &Vec<char>, b: &Vec<char>) -> Ordering {
    for i in 0..5 {
        match (a[i], b[i]) {
            (x, y) if x == y => continue,
            (x, y) if x == 'J' && y != 'J' => return Ordering::Less,
            (x, y) if x != 'J' && y == 'J' => return Ordering::Greater,
            (x, y) if x.is_alphabetic() && y.is_digit(10) => return Ordering::Greater,
            (x, y) if x.is_digit(10) && y.is_alphabetic() => return Ordering::Less,
            (x, y) if x.is_digit(10) && y.is_digit(10) => match (x, y) {
                (k, l) if k < l => return Ordering::Less,
                (k, l) if k > l => return Ordering::Greater,
                (_, _) => {
                    println!("-------------------------------------------");
                    return Ordering::Equal;
                }
            },
            (x, y) if x.is_alphabetic() && y.is_alphabetic() => match (x, y) {
                (k, l) if k == l => return Ordering::Equal,
                ('A', _) => return Ordering::Greater,
                ('K', 'A') => return Ordering::Less,
                ('K', 'Q') => return Ordering::Greater,
                ('K', 'J') => return Ordering::Greater,
                ('K', 'T') => return Ordering::Greater,
                ('Q', 'A') => return Ordering::Less,
                ('Q', 'K') => return Ordering::Less,
                ('Q', 'J') => return Ordering::Greater,
                ('Q', 'T') => return Ordering::Greater,
                ('J', 'A') => return Ordering::Less,
                ('J', 'K') => return Ordering::Less,
                ('J', 'Q') => return Ordering::Less,
                ('J', 'T') => return Ordering::Greater,
                ('T', _) => return Ordering::Less,
                (_, _) => {
                    println!("-------------------------------------------");
                    return Ordering::Equal;
                }
            },
            (_, _) => {
                println!("-------------------------------------------");
                return Ordering::Equal;
            }
        }
    }
    Ordering::Equal
}

fn parse_input(lines: &Vec<&str>) -> Vec<Vec<(String, i64)>> {
    let mut five: Vec<(String, i64)> = vec![];
    let mut four: Vec<(String, i64)> = vec![];
    let mut full: Vec<(String, i64)> = vec![];
    let mut three: Vec<(String, i64)> = vec![];
    let mut two: Vec<(String, i64)> = vec![];
    let mut one: Vec<(String, i64)> = vec![];
    let mut high: Vec<(String, i64)> = vec![];
    for line in lines.iter() {
        let split: Vec<&str> = line.split(" ").collect();
        let bid: i64 = split[1].parse().unwrap();
        let chars: HashSet<char> = HashSet::from_iter(split[0].chars().into_iter());
        let chars: Vec<char> = chars.into_iter().collect::<Vec<char>>();
        if chars.len() == 1 {
            five.push((split[0].to_string(), bid));
        } else if chars.len() == 2 {
            let c_cnt = split[0]
                .chars()
                .map(|x| if x == chars[0] { 1 } else { 0 })
                .sum::<i64>();
            if c_cnt == 4 || c_cnt == 1 {
                four.push((split[0].to_string(), bid));
            } else {
                full.push((split[0].to_string(), bid));
            }
        } else if chars.len() == 3 {
            let c_cnt = split[0]
                .chars()
                .map(|x| if x == chars[0] { 1 } else { 0 })
                .sum::<i64>();
            let d_cnt = split[0]
                .chars()
                .map(|x| if x == chars[1] { 1 } else { 0 })
                .sum::<i64>();
            let e_cnt = split[0]
                .chars()
                .map(|x| if x == chars[2] { 1 } else { 0 })
                .sum::<i64>();
            if c_cnt == 3 || d_cnt == 3 || e_cnt == 3 {
                three.push((split[0].to_string(), bid));
            } else {
                two.push((split[0].to_string(), bid));
            }
        } else if chars.len() == 4 {
            one.push((split[0].to_string(), bid));
        } else {
            high.push((split[0].to_string(), bid));
        }
    }
    vec![high, one, two, three, full, four, five]
}

fn parse_input_2(lines: &Vec<&str>) -> Vec<Vec<(String, i64)>> {
    let mut five: Vec<(String, i64)> = vec![];
    let mut four: Vec<(String, i64)> = vec![];
    let mut full: Vec<(String, i64)> = vec![];
    let mut three: Vec<(String, i64)> = vec![];
    let mut two: Vec<(String, i64)> = vec![];
    let mut one: Vec<(String, i64)> = vec![];
    let mut high: Vec<(String, i64)> = vec![];
    for line in lines.iter() {
        if line.contains("J") {
            let split: Vec<&str> = line.split(" ").collect();
            let bid: i64 = split[1].parse().unwrap();
            let chars: HashSet<char> =
                HashSet::from_iter(split[0].replace("J", "").chars().into_iter());
            let chars: Vec<char> = chars.into_iter().collect::<Vec<char>>();
            match chars.len() {
                4 => {
                    one.push((split[0].to_string(), bid));
                }
                3 => three.push((split[0].to_string(), bid)),
                2 => {
                    let x = split[0]
                        .chars()
                        .map(|x| if x == chars[0] { 1 } else { 0 })
                        .sum::<i64>();
                    let y = split[0]
                        .chars()
                        .map(|x| if x == chars[1] { 1 } else { 0 })
                        .sum::<i64>();
                    match (x, y) {
                        // J = 1
                        (3, 1) | (1, 3) => four.push((split[0].to_string(), bid)),
                        (2, 2) => full.push((split[0].to_string(), bid)),
                        // J = 2
                        (2, 1) | (1, 2) => four.push((split[0].to_string(), bid)),
                        // J = 3
                        (1, 1) => four.push((split[0].to_string(), bid)),
                        _ => println!("--------------------- {}", split[0].to_string()),
                    }
                }
                1 | 0 => {
                    five.push((split[0].to_string(), bid));
                }
                _ => println!("--------------------- {}", split[0].to_string()),
            }
        } else {
            let split: Vec<&str> = line.split(" ").collect();
            let bid: i64 = split[1].parse().unwrap();
            let chars: HashSet<char> = HashSet::from_iter(split[0].chars().into_iter());
            let chars: Vec<char> = chars.into_iter().collect::<Vec<char>>();
            match chars.len() {
                1 => five.push((split[0].to_string(), bid)),
                2 => {
                    let c_cnt = split[0]
                        .chars()
                        .map(|x| if x == chars[0] { 1 } else { 0 })
                        .sum::<i64>();
                    if c_cnt == 4 || c_cnt == 1 {
                        four.push((split[0].to_string(), bid));
                    } else {
                        full.push((split[0].to_string(), bid));
                    }
                }
                3 => {
                    let c_cnt = split[0]
                        .chars()
                        .map(|x| if x == chars[0] { 1 } else { 0 })
                        .sum::<i64>();
                    let d_cnt = split[0]
                        .chars()
                        .map(|x| if x == chars[1] { 1 } else { 0 })
                        .sum::<i64>();
                    let e_cnt = split[0]
                        .chars()
                        .map(|x| if x == chars[2] { 1 } else { 0 })
                        .sum::<i64>();
                    if c_cnt == 3 || d_cnt == 3 || e_cnt == 3 {
                        three.push((split[0].to_string(), bid));
                    } else {
                        two.push((split[0].to_string(), bid));
                    }
                }
                4 => one.push((split[0].to_string(), bid)),
                5 => high.push((split[0].to_string(), bid)),
                _ => println!("--------------------- {}", split[0].to_string()),
            }
        }
    }
    vec![high, one, two, three, full, four, five]
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day07/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let kinds = parse_input(&lines);
    let mut sorted: Vec<Vec<(String, i64)>> = vec![];
    for kind in kinds.clone().into_iter() {
        let mut tmp = kind.clone();
        tmp.sort_by(|a, b| hand_cmp(&a.0.chars().collect(), &b.0.chars().collect()));
        sorted.push(tmp);
    }
    //print_sorted(&sorted);
    let win = sum_ranked(&sorted);
    println!("Winnings: {}", win);
    println!();

    let kinds = parse_input_2(&lines);
    let mut sorted: Vec<Vec<(String, i64)>> = vec![];
    for kind in kinds.clone().into_iter() {
        let mut tmp = kind.clone();
        tmp.sort_by(|a, b| hand_cmp_2(&a.0.chars().collect(), &b.0.chars().collect()));
        sorted.push(tmp);
    }
    //print_sorted(&sorted);
    let win = sum_ranked(&sorted);
    println!("Winnings with Jokers: {}", win);
}

// 249426240 too high
// 249400220 just right
// 249240696 too low
