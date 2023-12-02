use std::fs;

fn parse_line(line: &str, red: i32, green: i32, blue: i32) -> (i32, i32) {
    let split: Vec<&str> = line.split(": ").collect();
    let game_id: i32 = split[0].split(" ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
    let draws: Vec<&str> = split[1].split("; ").collect();
    let mut bs = false;
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for draw in draws {
        let colors: Vec<&str> = draw.split(", ").collect();
        for color in colors {
            if color.contains("red") {
                let tmp = String::from_iter(color.chars().take_while(|x| *x != ' '))
                    .parse::<i32>()
                    .unwrap();
                if tmp > red && !bs {
                    bs = true;
                }
                if min_red < tmp {
                    min_red = tmp;
                }
            }
            if color.contains("green") {
                let tmp = String::from_iter(color.chars().take_while(|x| *x != ' '))
                    .parse::<i32>()
                    .unwrap();
                if tmp > green && !bs {
                    bs = true;
                }
                if min_green < tmp {
                    min_green = tmp;
                }
            }
            if color.contains("blue") {
                let tmp = String::from_iter(color.chars().take_while(|x| *x != ' '))
                    .parse::<i32>()
                    .unwrap();
                if tmp > blue && !bs {
                    bs = true;
                }
                if min_blue < tmp {
                    min_blue = tmp;
                }
            }
        }
    }
    (game_id, min_red * min_green * min_blue)
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day02/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let valid_games: Vec<(i32, i32)> = lines.iter().map(|x| parse_line(x, 12, 13, 14)).collect();
    println!("{}", valid_games.into_iter().map(|(_, x)| x).sum::<i32>());
}
