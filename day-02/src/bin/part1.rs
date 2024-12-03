use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let reports: Vec<String> = input
        .split("\n")
        .map(|line| line.to_string())
        .collect();

    let safe_reports: i32 = reports
        .iter()
        .map(|report| is_safe(report))
        .sum();

    println!("{}", safe_reports);
}

fn is_safe(report: &String) -> i32 {
    let levels: Vec<i32> = report.split_whitespace()
        .map(|level| level.parse().unwrap())
        .collect();

    let mut safe = true;
    let mut last_diff = 0;
    let mut last_level = 0;
    for (i, level) in levels.iter().enumerate() {
        if i != 0 {
            let diff = level - last_level;
            if diff == 0 || diff * last_diff < 0 || diff.abs() > 3 {
                safe = false;
                break;
            }
            last_level = *level;
            last_diff = diff;
        }
        else {last_level = *level};
    }
    return if safe{1} else{0};
}
