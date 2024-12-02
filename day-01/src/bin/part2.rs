use std::collections::HashMap;
use std::fs;

fn main() {
    let mut vec = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    let input = fs::read_to_string("./data/input.txt").unwrap();

    let mut flag = true;
    for word in input.split_whitespace() {
        let num_word: i32 = word.parse().unwrap();
        if flag {vec.push(num_word)}
        if !flag {*map.entry(num_word).or_insert(0) += 1}
        flag = !flag;
    }

    let distance: i32 = vec
        .iter()
        .map(|k| k*map.get(k).unwrap_or(&0))
        .sum();

    println!("{}", distance);
}
