use std::{fs, iter::zip, vec};

fn parse_input(lines: &Vec<&str>) -> Vec<(i64, i64)> {
    let times: String = lines[0].split(':').skip(1).collect::<Vec<&str>>()[0]
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let dist: String = lines[1].split(':').skip(1).collect::<Vec<&str>>()[0]
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}, {}", times, dist);
    let times: Vec<i64> = times
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let dist: Vec<i64> = dist
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    zip(times.into_iter(), dist.into_iter()).collect()
}

fn win_race_opts_triv(race: &(i64, i64)) -> i64 {
    let mut all: Vec<i64> = vec![];
    for i in 1..race.0 {
        let dist = (race.0 - i) * i;
        if dist > race.1 {
            all.push(i);
        }
    }
    all.len() as i64
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day06/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let races = parse_input(&lines);
    let mut opts = 1;
    for i in races {
        let win = win_race_opts_triv(&i);
        opts *= win;
    }
    println!("1) {}", opts);
    let lines: Vec<String> = lines
        .into_iter()
        .map(|line| line.replace(" ", ""))
        .collect();
    let mut new_lines: Vec<&str> = vec![];
    for line in lines.iter() {
        let tmp = line.as_str();
        new_lines.push(tmp);
    }
    let races = parse_input(&new_lines);
    let win = win_race_opts_triv(&races[0]);
    println!("2) {} ", win);
}
