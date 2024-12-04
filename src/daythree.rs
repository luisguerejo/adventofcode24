use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let reader = read_to_string("input.txt").expect("Should be able to read file");
    let mut number: u64 = 0;
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for line in reader.lines() {
        // Part one
        for (_, [first, second]) in mul_re.captures_iter(line).map(|c| c.extract()) {
            let first: u64 = first.parse().expect("Should be number");
            let second: u64 = second.parse().expect("Should be number");
            number += first * second;
        }
    }
    println!("part one: {number}");

    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();

    number = 0;
    let mut multi: bool = true;
    for line in reader.lines() {
        // Part two
        for capture in regex.captures_iter(line) {
            let str = capture.get(0).expect("Should be a first match").as_str();

            if str.contains("mul") && multi {
                let (_, [first, second]) = mul_re.captures(str).map(|c| c.extract()).unwrap();
                let first: u64 = first.parse().expect("Should be number");
                let second: u64 = second.parse().expect("Should be number");
                number += first * second;
                // number += first * second
            } else if str == "don't()" {
                multi = false;
            } else if str == "do()" {
                multi = true;
            }
        }
    }
    println!("part one: {number}");
}
