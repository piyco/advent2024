use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();

    let instructions: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();
    let mut state: bool = true;
    let total: i32 = instructions
        .iter()
        .map(|instruction| 
            if *instruction == "do()" {
                state = true;
                return 0;
            }
            else if *instruction == "don't()" {
                state = false;
                return 0;
            }
            else if state {
                handle_mul(instruction)
            }
            else {
                return 0;
            })
        .sum();

    println!("{}", total);
}

fn handle_mul(mul: &str)-> i32 { 
    let re = Regex::new(r"(\d{1,3}),(\d{1,3})").unwrap();
    let vals = re.captures(mul).unwrap();
    let val1 = vals.get(1).map_or("", |m| m.as_str());
    let val2 = vals.get(2).map_or("", |m| m.as_str());
    return val1.parse::<i32>().unwrap() * val2.parse::<i32>().unwrap();
}