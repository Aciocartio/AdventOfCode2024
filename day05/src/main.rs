use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

pub fn read_input(input: BufReader<File>, rules: &mut Vec<(u32, u32)>, pages: &mut Vec<Vec<u32>>) {
    for line in input.lines() {
        match line {
            Ok(line) => {
                if line.contains('|') {
                    let aux: Vec<u32> = line
                        .split('|')
                        .map(|element| element.parse().unwrap())
                        .collect();
                    rules.push((aux[0], aux[1]));
                } else if line.contains(',') {
                    pages.push(
                        line.split(',')
                            .map(|element| element.parse().unwrap())
                            .collect(),
                    );
                } else {
                    continue;
                }
            }
            Err(e) => {
                panic!("Error {} reading Lines", e)
            }
        }
    }
}

pub fn process_data_part1(rules: Vec<(u32, u32)>, pages: Vec<Vec<u32>>) -> u32 {
    let mut counter: u32 = 0;
    'pages: for page in pages {
        for pairs in &rules {
            let first = page.iter().position(|&x| x == pairs.0).unwrap_or(0);
            let second = page
                .iter()
                .position(|&x| x == pairs.1)
                .unwrap_or(usize::MAX);

            if first > second {
                continue 'pages;
            }
        }
        counter += page[page.len() / 2];
    }

    counter
}

pub fn process_data_part2(rules: Vec<(u32, u32)>, pages: Vec<Vec<u32>>) -> u32 {
    let mut counter: u32 = 0;
    let mut invalid_pages: Vec<Vec<u32>> = vec![];
    'pages: for page in pages {
        for pair in &rules {
            let first = page.iter().position(|&x| x == pair.0).unwrap_or(0);
            let second = page.iter().position(|&x| x == pair.1).unwrap_or(usize::MAX);

            if first > second {
                invalid_pages.push(page);
                continue 'pages;
            }
        }
    }
    
    for page in invalid_pages.iter_mut() {
        let mut reset = true;
        while reset {
            reset = false;
            for pair in &rules {
                let first: usize;
                let second: usize;
                if let Some(i) = page.iter().position(|&x| x == pair.0) {
                    first = i;
                } else {
                    continue;
                }
                if let Some(i) = page.iter().position(|&x| x == pair.1) {
                    second = i;
                } else {
                    continue;
                }

                if first > second {
                    let removed = page.remove(first);
                    page.insert(second, removed);
                    reset = true;
                    break;
                }
            }
        }
        counter += page[page.len() / 2];
    }

    counter
}
fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut rules: Vec<(u32, u32)> = vec![];
    let mut pages: Vec<Vec<u32>> = vec![];

    read_input(reader, &mut rules, &mut pages);
    println!("PART1: {}", process_data_part1(rules.clone(), pages.clone()));
    println!("PART2: {}", process_data_part2(rules, pages));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let file = File::open("test_input").unwrap();
        let mut rules: Vec<(u32, u32)> = vec![];
        let mut pages: Vec<Vec<u32>> = vec![];
        read_input(BufReader::new(file), &mut rules, &mut pages);

        assert_eq!(rules, vec![(1, 4), (2, 5), (3, 6), (9, 7)]);
        assert_eq!(pages, vec![vec![1, 3, 5], vec![4, 2, 7, 9]]);
    }

    #[test]
    fn test_logic_part_1() {
        let rules: Vec<(u32, u32)> = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let pages: Vec<Vec<u32>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        assert_eq!(process_data_part1(rules, pages), 143);
    }

    #[test]
    fn test_logic_part_2() {
        let rules: Vec<(u32, u32)> = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let pages: Vec<Vec<u32>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        assert_eq!(process_data_part2(rules, pages), 123);
    }
}
