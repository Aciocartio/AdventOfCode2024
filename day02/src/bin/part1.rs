use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    for line in reader.lines() {
        match line {
            Ok(i) => {
                let levels_str: Vec<&str> = i.split_whitespace().collect();
                let levels_u32: Vec<u32> =
                    levels_str.clone().into_iter().map(|x| x.parse().unwrap()).collect();
                if is_decreasing(&levels_u32) || is_increasing(&levels_u32) {
                    sum += 1;
                }
            }
            _ => (),
        }
    }

    println!("Number of safe reports = {}", sum);
}

fn is_increasing(vec: &Vec<u32>) -> bool {
    vec.windows(2).all(|w| w[0] < w[1] && w[1]-w[0]<=3)
}

fn is_decreasing(vec: &Vec<u32>) -> bool {
    vec.windows(2).all(|w| w[0] > w[1] && w[0]-w[1]<=3)
}
