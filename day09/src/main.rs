use std::fs;

fn parse_input(lines: Vec<&str>) -> Vec<Vec<i32>> {
    let mut data: Vec<Vec<i32>> = vec![];
    for line in lines {
        data.push(line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect())
    }
    data
}

fn print_input(data: &Vec<Vec<i32>>) {
    for l in data {
        for v in l {
            print!("{} ", v);
        }
        println!();
    }
}

fn get_diffs(data: &Vec<i32>) -> Vec<i32> {
    let mut diffs: Vec<i32> = vec![];
    for i in 0..(data.len() - 1) {
        diffs.push(data[i + 1] - data[i]);
    }
    diffs
}

fn reduce(data: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut reduce_seq: Vec<Vec<i32>> = vec![];
    let mut reduced = data.clone();
    while !reduced.iter().all(|x| *x == 0) {
        reduce_seq.push(reduced.clone());
        reduced = get_diffs(&reduced);
    }
    reduce_seq
}

fn expand(data: &Vec<Vec<i32>>) -> i32 {
    let mut accum: i32 = 0;
    for i in data {
        accum += i.last().unwrap();
    }
    accum
}

fn implode(data: &Vec<Vec<i32>>) -> i32 {
    let mut last: i32 = data.last().unwrap().first().unwrap().clone();
    for i in (0..data.len() - 1).rev() {
        //print!("{}, {}, ", data[i].first().unwrap(), last);
        last = data[i].first().unwrap() - last;
        //println!("{}", last)
    }
    last
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day09/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let input = parse_input(lines);
    //print_input(&input);
    let accum: i32 = input
        .clone()
        .into_iter()
        .map(|x| {
            //println!("{:?}", x);
            expand({
                let tmp = reduce(&x);
                //print_input(&tmp);
                //println!();
                &tmp.clone()
            })
        })
        .sum();
    println!("Extrapolations: {}", accum);
    let accum: i32 = input
        .into_iter()
        .map(|x| {
            //println!("{:?}", x);
            implode({
                let tmp = reduce(&x);
                //print_input(&tmp);
                //println!();
                &tmp.clone()
            })
        })
        .sum();
    println!("Backtrapolations: {}", accum);
}
