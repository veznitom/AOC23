use std::{fs, vec};

fn parse_seeds(
    lines: &Vec<&str>,
) -> (
    Vec<i64>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
) {
    let mut maps: Vec<Vec<&str>> = vec![];
    maps.push(vec![]);
    for line in lines.iter() {
        if line.is_empty() || *line == "\n" {
            maps.push(vec![])
        } else {
            maps.last_mut().unwrap().push(line);
        }
    }
    let seeds = maps[0].clone();
    let seed_soil = maps[1].clone();
    let soil_fert = maps[2].clone();
    let fert_watr = maps[3].clone();
    let watr_ligh = maps[4].clone();
    let ligh_temp = maps[5].clone();
    let temp_humd = maps[6].clone();
    let humd_loct = maps[7].clone();

    let seeds = seeds.first().unwrap().split(": ").collect::<Vec<&str>>()[1]
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (
        seeds,
        parse_maps(&seed_soil),
        parse_maps(&soil_fert),
        parse_maps(&fert_watr),
        parse_maps(&watr_ligh),
        parse_maps(&ligh_temp),
        parse_maps(&temp_humd),
        parse_maps(&humd_loct),
    )
}

fn parse_maps(map: &Vec<&str>) -> Vec<(i64, i64, i64)> {
    let vals = map.split_at(1).1;
    let mut funcs: Vec<(i64, i64, i64)> = vec![];
    for v in vals {
        let tmp: Vec<i64> = v.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        funcs.push((tmp[0], tmp[1], tmp[2]));
    }
    funcs
}

fn get_low_loc(seeds: &Vec<i64>, maps: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    let mut lowest = i64::MAX;
    for seed in seeds.iter() {
        let mut tmp = *seed;
        for i in 0..maps.len() {
            for m in maps[i].iter() {
                if m.1 <= tmp && tmp < m.1 + m.2 {
                    tmp = m.0 + (tmp - m.1);
                    break;
                }
            }
        }
        if lowest > tmp {
            lowest = tmp;
        }
    }
    lowest
}

fn is_range_empty(range: (i64, i64)) -> bool {
    if range.0 < 0 || range.1 < 0 {
        true
    } else {
        false
    }
}

fn split_range(range: (i64, i64), map: (i64, i64, i64)) -> ((i64, i64), (i64, i64), (i64, i64)) {
    let mut before: (i64, i64) = (-1, -1);
    let mut inside: (i64, i64) = (-1, -1);
    let mut after: (i64, i64) = (-1, -1);
    // whole before
    if range.1 < map.1 {
        before = range;
    }
    // whole inside
    else if map.1 <= range.0 && range.1 < (map.1 + map.2) {
        inside = (map.0 + range.0 - map.1, map.0 + range.1 - map.1);
    }
    // whole after
    else if (map.1 + map.2) <= range.0 {
        after = range;
    }
    // before leftover
    else if range.0 < map.1 && range.1 < map.1 + map.2 {
        before = (range.0, map.1 - 1);
        inside = (map.0, map.0 + range.1 - map.1);
    }
    // after leftover
    else if map.1 <= range.0 && range.1 >= map.1 + map.2 {
        inside = (map.0 + range.0 - map.1, map.0 + map.2 - 1);
        after = (map.1 + map.2, range.1);
    }
    // both leftover
    else if range.0 < map.1 && range.1 >= map.1 + map.2 {
        before = (range.0, map.1 - 1);
        inside = (map.0, map.0 + map.2 - 1);
        after = (map.1 + map.2, range.1);
    }
    (before, inside, after)
}

fn map_range(
    range: (i64, i64),
    maps: &Vec<Vec<(i64, i64, i64)>>,
    map_index: i64,
    rule_index: i64,
) -> i64 {
    if map_index == 6 {
        let map = &maps[map_index as usize];
        if rule_index as usize == map.len() - 1 {
            let mut lowest = i64::MAX;
            let rule = map[rule_index as usize];
            let mut mins: Vec<i64> = vec![];
            let ranges = split_range(range, rule);
            if !is_range_empty(ranges.0) {
                mins.push(ranges.0 .0);
            }
            if !is_range_empty(ranges.1) {
                mins.push(ranges.1 .0);
            }
            if !is_range_empty(ranges.2) {
                mins.push(ranges.2 .0);
            }
            if mins.len() > 0 {
                let min = mins.into_iter().min().unwrap();
                if min < lowest {
                    lowest = min;
                }
            }
            lowest
        } else {
            let mut lowest = i64::MAX;
            let rule = map[rule_index as usize];
            let mut mins: Vec<i64> = vec![];
            let ranges = split_range(range, rule);
            if !is_range_empty(ranges.0) {
                mins.push(map_range(ranges.0, maps, map_index, rule_index + 1));
            }
            if !is_range_empty(ranges.1) {
                mins.push(ranges.1 .0);
            }
            if !is_range_empty(ranges.2) {
                mins.push(map_range(ranges.2, maps, map_index, rule_index + 1));
            }
            if mins.len() > 0 {
                let min = mins.into_iter().min().unwrap();
                if min < lowest {
                    lowest = min;
                }
            }
            lowest
        }
    } else {
        let map = &maps[map_index as usize];
        if rule_index as usize == map.len() - 1 {
            let mut lowest = i64::MAX;
            let rule = map[rule_index as usize];
            let mut mins: Vec<i64> = vec![];
            let ranges = split_range(range, rule);
            if !is_range_empty(ranges.0) {
                mins.push(map_range(ranges.0, maps, map_index + 1, 0));
            }
            if !is_range_empty(ranges.1) {
                mins.push(map_range(ranges.1, maps, map_index + 1, 0));
            }
            if !is_range_empty(ranges.2) {
                mins.push(map_range(ranges.2, maps, map_index + 1, 0));
            }
            if mins.len() > 0 {
                let min = mins.into_iter().min().unwrap();
                if min < lowest {
                    lowest = min;
                }
            }
            lowest
        } else {
            let mut lowest = i64::MAX;
            let rule = map[rule_index as usize];
            let mut mins: Vec<i64> = vec![];
            let ranges = split_range(range, rule);
            if !is_range_empty(ranges.0) {
                mins.push(map_range(ranges.0, maps, map_index, rule_index + 1));
            }
            if !is_range_empty(ranges.1) {
                mins.push(map_range(ranges.1, maps, map_index + 1, 0));
            }
            if !is_range_empty(ranges.2) {
                mins.push(map_range(ranges.2, maps, map_index, rule_index + 1));
            }
            if mins.len() > 0 {
                let min = mins.into_iter().min().unwrap();
                if min < lowest {
                    lowest = min;
                }
            }
            lowest
        }
    }
}

fn get_low_loc_range(seeds: &Vec<(i64, i64)>, maps: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    let mut lowest = i64::MAX;
    for range in seeds.iter() {
        let min = map_range(*range, maps, 0, 0);
        if min < lowest {
            lowest = min;
        }
    }
    lowest
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day05/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let input = parse_seeds(&lines);
    let maps = vec![
        input.1, input.2, input.3, input.4, input.5, input.6, input.7,
    ];
    println!("Lowest location: {}", get_low_loc(&input.0, &maps));
    let mut ranges: Vec<(i64, i64)> = vec![];
    for i in (0..(input.0.len())).step_by(2) {
        ranges.push((input.0[i], input.0[i] + input.0[i + 1]));
    }
    println!(
        "Lowest location in ranges: {}",
        get_low_loc_range(&ranges, &maps)
    );
}
