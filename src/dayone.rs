use std::cmp::Ordering;
use std::{fs::read_to_string, u32};

fn main() {
    let reader = read_to_string("input.txt").expect("Should be able to read file");

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(tokens.len(), 2);
        let first: u32 = tokens[0].parse().expect("Should be able to cast");
        let second: u32 = tokens[1].parse().expect("Should be able to cast");
        left.push(first);
        right.push(second);
    }
    left.sort();
    right.sort();

    let mut distance: u32 = 0;
    for (left, right) in left.iter().zip(right.iter()) {
        match left.cmp(right) {
            Ordering::Greater => distance += left - right,
            Ordering::Less => distance += right - left,
            Ordering::Equal => continue,
        }
    }

    println!("part one: {distance}");

    distance = 0;
    for value in left {
        let multiplier = right.iter().filter(|&x| *x == value).count();
        distance += value * multiplier as u32;
    }

    println!("part two: {distance}");
}
