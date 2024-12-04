use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: i32 = 0;
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32>= vec![];

    for line in reader.lines() {
        match line {
            Ok(i) => {
                
                let pair: Vec<&str> = i.split_whitespace().collect();
                left.push(pair[0].parse().unwrap());
                right.push(pair[1].parse().unwrap());
            }
            _ => (),
        }
    }
    left.sort();
    right.sort();

    for i in 0..left.len()
    {
        sum += (left[i]-right[i]).abs();
    }

    println!("Soma é {}", sum);
}
