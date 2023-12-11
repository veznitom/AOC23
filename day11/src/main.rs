use std::{collections::HashSet, fs};

fn expand(lines: &Vec<&str>) -> Vec<Vec<usize>> {
    let mut exp: Vec<Vec<usize>> = vec![];
    let mut rows: Vec<usize> = vec![];
    let mut columns: Vec<usize> = vec![];
    for line in lines {
        rows.push(if line.contains('#') { 1 } else { 0 });
    }
    for col in 0..lines[0].len() {
        let mut sum = 0;
        for row in lines {
            if row.chars().clone().collect::<Vec<char>>()[col] == '#' {
                sum += 1;
            }
        }
        columns.push(sum);
    }
    exp.push(rows);
    exp.push(columns);
    exp
}

fn parse_input(lines: &Vec<&str>, con: usize) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let gaps: Vec<Vec<usize>> = expand(&lines);
    //println!("Rows      {:?}", gaps[0]);
    //println!("Columns   {:?}", gaps[1]);
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y].chars().collect::<Vec<char>>()[x] == '#' {
                let x_cord: usize = gaps[1]
                    .split_at(x)
                    .0
                    .into_iter()
                    .map(|g| if *g == 0 { 1 } else { 0 })
                    .sum::<usize>()
                    * con
                    + x;
                let y_cord: usize = gaps[0]
                    .split_at(y)
                    .0
                    .into_iter()
                    .map(|g| if *g == 0 { 1 } else { 0 })
                    .sum::<usize>()
                    * con
                    + y;
                galaxies.push((y_cord, x_cord));
            }
        }
    }
    galaxies
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day11/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let galaxies: Vec<(usize, usize)> = parse_input(&lines, 1);
    //println!("Galaxies {:?}", galaxies);
    let mut g_pairs: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    for i in galaxies.iter() {
        for j in galaxies.iter() {
            if i != j && !g_pairs.contains(&(j.clone(), i.clone())) {
                g_pairs.insert((i.clone(), j.clone()));
            }
        }
    }
    println!("Sum {}",g_pairs.into_iter().map(|x| ((x.0.0 as i64 - x.1.0 as i64).abs() + (x.0.1 as i64 - x.1.1 as i64).abs())).sum::<i64>());

    let galaxies: Vec<(usize, usize)> = parse_input(&lines, 1000000-1);
    //println!("Galaxies {:?}", galaxies);
    let mut g_pairs: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    for i in galaxies.iter() {
        for j in galaxies.iter() {
            if i != j && !g_pairs.contains(&(j.clone(), i.clone())) {
                g_pairs.insert((i.clone(), j.clone()));
            }
        }
    }
    println!("Sum {}",g_pairs.into_iter().map(|x| ((x.0.0 as i64 - x.1.0 as i64).abs() + (x.0.1 as i64 - x.1.1 as i64).abs())).sum::<i64>());
}
