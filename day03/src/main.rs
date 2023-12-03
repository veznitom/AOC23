use std::{collections::HashSet, fs, ops::Add, usize, vec};
#[derive(Clone, Copy)]
enum Parts {
    Number(i32),
    Symbol(char),
    None,
}

fn match_part(part: &Parts) -> i32 {
    match part {
        Parts::Number(num) => *num,
        Parts::Symbol(c) => (*c).to_digit(10).unwrap() as i32,
        Parts::None => 0,
    }
}

fn get_link(part: &Parts) -> i32 {
    match part {
        Parts::Number(num) => *num,
        Parts::Symbol(c) => 0,
        Parts::None => 0,
    }
}

fn is_symbol(part: &Parts) -> bool {
    match part {
        Parts::Number(_) => false,
        Parts::Symbol(_) => true,
        Parts::None => false,
    }
}

fn is_number(part: &Parts) -> bool {
    match part {
        Parts::Number(_) => true,
        Parts::Symbol(_) => false,
        Parts::None => false,
    }
}

fn print_part(part: &Parts, vals: &Vec<i32>) {
    match part {
        Parts::Number(num) => print!("{} ", vals[*num as usize]),
        Parts::Symbol(sym) => print!("{}", sym),
        Parts::None => print!(". "),
    }
}

fn parse_input(lines: Vec<&str>) -> (Vec<Vec<Parts>>, Vec<i32>) {
    let mut vals: Vec<i32> = vec![];
    let mut parts: Vec<Vec<Parts>> = vec![];
    let mut link_index: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        parts.push(vec![]);
        parts[i].push(Parts::None);
        let mut num_skip = false;
        'inner: for (j, c) in line.chars().enumerate() {
            if num_skip && c.is_digit(10) {
                let tmp = parts[i].last().unwrap().clone();
                parts[i].push(tmp);
                continue 'inner;
            } else {
                num_skip = false;
            }
            if c == '.' {
                parts[i].push(Parts::None);
                continue 'inner;
            }
            if c.is_digit(10) {
                num_skip = true;
                let rest: i32 = String::from_iter(
                    line[j..]
                        .chars()
                        .into_iter()
                        .take_while(|x| (*x).is_digit(10)),
                )
                .parse::<i32>()
                .unwrap();
                vals.push(rest);
                parts[i].push(Parts::Number(link_index as i32));
                link_index += 1;
                continue 'inner;
            }
            parts[i].push(Parts::Symbol(c));
        }
        parts[i].push(Parts::None);
    }
    let mut tmp = vec![];
    tmp.resize_with(parts[0].len(), || Parts::None);
    parts.insert(0, tmp);
    let mut tmp = vec![];
    tmp.resize_with(parts[0].len(), || Parts::None);
    parts.push(tmp);
    (parts, vals)
}

fn get_part_nums(parts: &Vec<Vec<Parts>>, vals: &Vec<i32>) -> i32 {
    let mut cord: HashSet<usize> = HashSet::new();
    for i in 1..(parts.len() - 1) {
        for j in 1..(parts[i].len() - 1) {
            //print_part(&parts[i][j], &vals);
            match parts[i][j] {
                Parts::Symbol(_) => {
                    if is_number(&parts[i - 1][j - 1]) {
                        cord.insert(get_link(&parts[i - 1][j - 1]) as usize);
                    }
                    if is_number(&parts[i - 1][j]) {
                        cord.insert(get_link(&parts[i - 1][j]) as usize);
                    }
                    if is_number(&parts[i - 1][j + 1]) {
                        cord.insert(get_link(&parts[i - 1][j + 1]) as usize);
                    }
                    if is_number(&parts[i][j - 1]) {
                        cord.insert(get_link(&parts[i][j - 1]) as usize);
                    }
                    if is_number(&parts[i][j + 1]) {
                        cord.insert(get_link(&parts[i][j + 1]) as usize);
                    }
                    if is_number(&parts[i + 1][j - 1]) {
                        cord.insert(get_link(&parts[i + 1][j - 1]) as usize);
                    }
                    if is_number(&parts[i + 1][j]) {
                        cord.insert(get_link(&parts[i + 1][j]) as usize);
                    }
                    if is_number(&parts[i + 1][j + 1]) {
                        cord.insert(get_link(&parts[i + 1][j + 1]) as usize);
                    }
                }
                Parts::Number(_) | Parts::None => continue,
            }
        }
//        println!()
    }
    cord.into_iter().map(|i| vals[i]).sum()
}

fn sum_parts(parts: &Vec<Vec<Parts>>, vals: &Vec<i32>) -> i32 {
    let mut times: Vec<i32> = vec![];
    for i in 1..(parts.len() - 1) {
        for j in 1..(parts[i].len() - 1) {
            //print_part(&parts[i][j], &vals);
            match parts[i][j] {
                Parts::Symbol(sym) => {
                    if sym == '*' {
                        let mut cord: HashSet<usize> = HashSet::new();
                        if is_number(&parts[i - 1][j - 1]) {
                            cord.insert(get_link(&parts[i - 1][j - 1]) as usize);
                        }
                        if is_number(&parts[i - 1][j]) {
                            cord.insert(get_link(&parts[i - 1][j]) as usize);
                        }
                        if is_number(&parts[i - 1][j + 1]) {
                            cord.insert(get_link(&parts[i - 1][j + 1]) as usize);
                        }
                        if is_number(&parts[i][j - 1]) {
                            cord.insert(get_link(&parts[i][j - 1]) as usize);
                        }
                        if is_number(&parts[i][j + 1]) {
                            cord.insert(get_link(&parts[i][j + 1]) as usize);
                        }
                        if is_number(&parts[i + 1][j - 1]) {
                            cord.insert(get_link(&parts[i + 1][j - 1]) as usize);
                        }
                        if is_number(&parts[i + 1][j]) {
                            cord.insert(get_link(&parts[i + 1][j]) as usize);
                        }
                        if is_number(&parts[i + 1][j + 1]) {
                            cord.insert(get_link(&parts[i + 1][j + 1]) as usize);
                        }
                        if cord.len() == 2 {
                            let mut acc = 1;
                            for i in cord {
                                acc *= vals[i];
                            }
                            times.push(acc);
                        }
                    }
                }
                Parts::Number(_) | Parts::None => continue,
            }
        }
        //println!()
    }
    times.into_iter().sum::<i32>()
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day03/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let parts: Vec<Vec<Parts>>;
    let vals: Vec<i32>;
    (parts, vals) = parse_input(lines);
    println!("{}", get_part_nums(&parts, &vals));
    println!("{}", sum_parts(&parts, &vals));
}
