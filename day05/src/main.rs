use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut rules: Vec<[u32; 2]> = vec![];
    let mut sum: u32 = 0;

    for line in reader.lines() {
        match line {
            Ok(i) => {
                if i.contains('|') {
                    let numbers: Vec<&str> = i.split('|').collect();
                    let first: u32 = numbers[0].parse().unwrap();
                    let second: u32 = numbers[1].parse().unwrap();
                    rules.push([first, second]);
                } else if i.contains(',') {
                    let update: Vec<u32> = i
                        .split(',')
                        .map(|x| -> u32 { x.parse().unwrap() })
                        .collect();
                    let mut position_map: HashMap<u32, u32> = HashMap::new();
                    for (pos, element) in update.iter().enumerate() {
                        position_map.insert(*element, pos as u32);
                    }
                    let mut flag = true;
                    for rule in &rules {
                        if update.contains(&rule[0]) && update.contains(&rule[1]) {
                            if position_map[&rule[1]] > position_map[&rule[0]] {
                                continue;
                            } else {
                                flag = false
                            }
                        }
                    }
                    if flag {
                        let pos = ((update.len() as f64) / (2 as f64).ceil()) as usize;
                        sum += update[pos];
                    }
                }
            }
            _ => (),
        }
    }
    println!("{}", sum);
}
