use std::fs;
#[derive(Default)]
struct Ticket {
    id: usize,
    left: Vec<i32>,
    right: Vec<i32>,
    wins: usize,
}

fn print_ticket(ticket: &Ticket) {
    print!("{}: ", ticket.id);
    for left in ticket.left.iter() {
        print!("{:02} ", left)
    }
    print!("| ");
    for right in ticket.right.iter() {
        print!("{:02} ", right)
    }
    println!("  : {}", ticket.wins);
}

fn print_tickets(tickets: &Vec<Ticket>) {
    for i in tickets.iter() {
        print!("    ");
        print_ticket(&i);
    }
}

fn get_points(left: &Vec<i32>, right: &Vec<i32>) -> usize {
    let mut wins: usize = 0;
    for r in right {
        if left.contains(&r) {
            wins += 1;
        }
    }
    wins
}

fn to_ticket(line: &str) -> Ticket {
    let values = line
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let values = values.split(':').collect::<Vec<&str>>();
    let id: usize = values[0].split(' ').collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();
    let values = values[1].split('|').collect::<Vec<&str>>();
    let left: Vec<i32> = values[0]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| (*x).parse::<i32>().unwrap())
        .collect();
    let right: Vec<i32> = values[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| (*x).parse::<i32>().unwrap())
        .collect();
    let wins = get_points(&left, &right);
    Ticket {
        id,
        left,
        right,
        wins,
    }
}

fn sum_wins(tickets: &Vec<Ticket>) -> usize {
    let mut sum: usize = 0;
    for ticket in tickets {
        sum += if ticket.wins == 0 {
            0
        } else {
            (2 as i32).pow((ticket.wins - 1) as u32) as usize
        };
    }
    sum
}

fn sum_ticktets(ticket: &Ticket, index: usize, tickets: &Vec<Ticket>) -> usize {
    //print_ticket(ticket);
    if index == tickets.len() || tickets[index].wins == 0 {
        return 0;
    }
    let mut total = ticket.wins;
    for i in 1..ticket.wins + 1 {
        total += sum_ticktets(&tickets[i + index], index + i, tickets);
    }
    total
}

fn main() {
    let file = fs::read_to_string("/home/tomasv/Projects/AOC23/day04/input").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let tickets: Vec<Ticket> = lines
        .into_iter()
        .map(|x| to_ticket(x))
        .collect::<Vec<Ticket>>();
    //print_tickets(&tickets);
    println!("Win score: {}", sum_wins(&tickets));
    let mut total = 0;
    for (i, ticket) in tickets.iter().enumerate() {
        total += sum_ticktets(ticket, i, &tickets);
    }
    println!("Tickets total: {}", total + tickets.len());
}
