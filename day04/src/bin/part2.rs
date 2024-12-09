use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        match line {
            Ok(i) => {
                let aux: Vec<char> = i.chars().collect();
                matrix.push(aux);
            }
            _ => (),
        }
    }

    let mut sum: u32 = 0;
    let x_max = matrix.len();
    for i in 0..x_max {
        let y_max = matrix[i].len();

        let check_pos =
            |x: usize, y: usize| return x > 0 && x < x_max-1 as usize && y > 0 && y < y_max-1 as usize;

        for j in 0..y_max {
            if matrix[i][j] == 'A' && check_pos(i, j) {
                if (matrix[i - 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S')
                    || (matrix[i - 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M')
                {
                    if (matrix[i + 1][j - 1] == 'M' && matrix[i - 1][j + 1] == 'S')
                        || (matrix[i + 1][j - 1] == 'S' && matrix[i - 1][j + 1] == 'M')
                    {
                        sum += 1;
                    }
                }
            }
        }
    }
    println!("Final sum = {}", sum);
}
