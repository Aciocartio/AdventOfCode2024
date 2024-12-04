use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    for line in reader.lines() {
        match line {
            Ok(i) => {
                let levels_str: Vec<&str> = i.split_whitespace().collect();
                let levels_u32: Vec<u32> = levels_str
                    .clone()
                    .into_iter()
                    .map(|x| x.parse().unwrap())
                    .collect();

                if is_decreasing_flex(&levels_u32) || is_increasing_flex(&levels_u32)
                {
                    sum += 1;
                }
            }
            _ => (),
        }
    }

    println!("Number of safe reports = {}", sum);
}

fn is_increasing_flex(vec: &Vec<u32>) -> bool {
    let mut position: usize = 0;
    for i in 0..vec.len() - 1 {
        if vec[i] >= vec[i + 1] || vec[i + 1] - vec[i] > 3 {
            position = i + 1;
            break;
        }
    }
    if position != 0 {
        let mut clone1 = vec.clone();
        clone1.remove(position);
        let mut clone2 = vec.clone();
        clone2.remove(position - 1);

        return is_increasing(&clone1) || is_increasing(&clone2);
    } else {
        true
    }
}

fn is_increasing(vec: &Vec<u32>) -> bool {
    vec.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
}

fn is_decreasing_flex(vec: &Vec<u32>) -> bool {
    let mut position: usize = 0;
    for i in 0..vec.len() - 1 {
        if vec[i] <= vec[i + 1] || vec[i] - vec[i + 1] > 3 {
            position = i + 1;
            break;
        }
    }
    if position != 0 {
        let mut clone1 = vec.clone();
        clone1.remove(position);
        let mut clone2 = vec.clone();
        clone2.remove(position - 1);

        return is_decreasing(&clone1) || is_decreasing(&clone2);
    } else {
        true
    }
}

fn is_decreasing(vec: &Vec<u32>) -> bool {
    vec.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3)
}
