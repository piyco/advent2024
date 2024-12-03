use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let muls: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();
    let total: i32 = muls
        .iter()
        .map(|mul| handle_mul(mul))
        .sum();

    println!("{}", total);
}

fn handle_mul(mul: &str) ->i32 { 
    let re = Regex::new(r"(\d{1,3}),(\d{1,3})").unwrap();
    let vals = re.captures(mul).unwrap();
    let val1 = vals.get(1).map_or("", |m| m.as_str());
    let val2 = vals.get(2).map_or("", |m| m.as_str());
    return val1.parse::<i32>().unwrap() * val2.parse::<i32>().unwrap();
}