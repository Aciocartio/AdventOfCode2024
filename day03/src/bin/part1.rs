use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(&pattern).unwrap();

    for line in reader.lines() {
        match line {
            Ok(i) => {
                for cap in re.captures_iter(&i) {
                    let num1: u32 = cap[1].parse().unwrap();
                    let num2: u32 = cap[2].parse().unwrap();
                    sum += num1 * num2;
                }
            }
            _ => (),
        }
    }

    println!("Final sum = {}", sum);
}