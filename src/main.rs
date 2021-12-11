use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let lines = convert_lines(&contents);
    println!("{}", solve_day1a(&lines));
    println!("{}", solve_day1b(&lines));
}

fn convert_lines(lines: &String) -> Vec<i32> {
    let mut out = Vec::new();
    for l in lines.trim().split("\n") {
        out.push(l.trim().parse().unwrap());
    }
    out
}

fn solve_day1a(depths: &Vec<i32>) -> i32 {
    if depths.len() == 0 {
        return 0;
    }
    let mut last = depths[0];
    let mut count = 0;
    for i in depths {
        if *i > last {
            count += 1;
        }
        last = *i
    }
    count
}

fn solve_day1b(depths: &Vec<i32>) -> i32 {
    if depths.len() == 0 {
        return 0;
    }
    let head = depths.iter().skip(3);
    let mut tail = depths.iter();
    let mut count = 0;
    for i in head {
        if i > tail.next().unwrap() {
            count += 1;
        }
    }
    count
}
