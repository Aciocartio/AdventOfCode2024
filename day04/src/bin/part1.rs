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

    let moves: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];

    let mut sum: u32 = 0;
    let x_max = matrix.len();
    for i in 0..x_max {
        let y_max = matrix[i].len();

        let check_pos =
            |x: i32, y: i32| return x >= 0 && x < x_max as i32 && y >= 0 && y < y_max as i32;

        for j in 0..y_max {
            if matrix[i][j] == 'X' {
                for (x_move, y_move) in &moves {
                    let mut aux_i = i as i32 + x_move;
                    let mut aux_j = j as i32 + y_move;

                    if check_pos(aux_i, aux_j) {
                        if matrix[aux_i as usize][aux_j as usize] == 'M' {
                            aux_i += x_move;
                            aux_j += y_move;
                            {
                                if check_pos(aux_i, aux_j) {
                                    if matrix[aux_i as usize][aux_j as usize] == 'A' {
                                        aux_i += x_move;
                                        aux_j += y_move;
                                        {
                                            if check_pos(aux_i, aux_j) {
                                                if matrix[aux_i as usize][aux_j as usize] == 'S' {
                                                    sum += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Final sum = {}", sum);
}

fn check_pos(x: i8, y: i8, x_max: i8, y_max: i8) -> bool {
    x >= 0 && x < x_max && y >= 0 && y < y_max
}
