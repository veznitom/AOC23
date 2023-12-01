use std::fs;

fn filter_nums(line: &str) -> i32 {
    let removed: Vec<u32> = line
        .chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    (removed.get(0).unwrap() * 10 + removed.last().unwrap()) as i32
}

fn filter_nuns(line: &str) -> i32 {
    let texted = line
        .replace("1", "_one_")
        .replace("2", "_two_")
        .replace("3", "_three_")
        .replace("4", "_four_")
        .replace("5", "_five_")
        .replace("6", "_six_")
        .replace("7", "_seven_")
        .replace("8", "_eight_")
        .replace("9", "_nine_") + "*****";
    let mut ints: Vec<i32> = vec![];
    for window in texted.bytes().collect::<Vec<u8>>().windows(5) {
        let tmp = String::from_utf8(window.to_vec()).unwrap();
        ints.push(if tmp.starts_with("one") {
            1
        } else if tmp.starts_with("two") {
            2
        } else if tmp.starts_with("three") {
            3
        } else if tmp.starts_with("four") {
            4
        } else if tmp.starts_with("five") {
            5
        } else if tmp.starts_with("six") {
            6
        } else if tmp.starts_with("seven") {
            7
        } else if tmp.starts_with("eight") {
            8
        } else if tmp.starts_with("nine") {
            9
        } else {
            0
        })
    }
    let fints: Vec<i32> = ints.into_iter().filter(|x| *x > 0).collect();
    //println!("{:?}", fints);
    (fints.get(0).unwrap() * 10 + fints.last().unwrap()) as i32
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day01/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let linex = lines.clone();
    let num: i32 = lines.into_iter().map(filter_nums).sum();
    let mum: i32 = linex.into_iter().map(filter_nuns).sum();
    println!("{},{}", num, mum)
}
