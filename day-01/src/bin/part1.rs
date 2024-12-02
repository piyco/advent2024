use std::fs;

fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    let input = fs::read_to_string("./data/input.txt").unwrap();

    let mut flag = true;
    for word in input.split_whitespace() {
        let num_word: i32 = word.parse().unwrap();
        if flag {vec1.push(num_word)} else {vec2.push(num_word)};
        flag = !flag;
    }

    vec1.sort();
    vec2.sort();
    let distance: i32 = vec1
        .iter()
        .zip(vec2)
        .map(|(max1, max2)| (max1 - max2).abs())
        .sum();

    println!("{}", distance);
}
