use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let mut heap1: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap2: BinaryHeap<i32> = BinaryHeap::new();

    let reader = BufReader::new(File::open("./data/input.txt").expect("Cannot open file.txt"));

    for line in reader.lines() {

        let mut flag = true;
        for word in line.unwrap().split_whitespace() {
            let num_word: i32 = word.parse().unwrap();
            if flag {heap1.push(num_word)} else {heap2.push(num_word)};
            flag = false;
        }
    }

    let mut total: i32 = 0;
    while !heap1.is_empty() {
        if let (Some(max1), Some(max2)) = (heap1.pop(), heap2.pop()) {
            total += (max1 - max2).abs();
        }
    }

    println!("{}", total);
}
