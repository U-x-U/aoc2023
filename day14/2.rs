// instinct: does such cycles have a period?
//   1. # of possible state after a cycle: don't know how to estimate...
//     but it is the only promising approach I came up with
//   2. how to represent a state efficiently?
//     a vec of number of 'O's to the west of each '#'
use std::collections::HashMap;
const COUNT: usize = 1_000_000_000;
fn main() {
    // let mut matrix = include_str!("test.txt")
    let mut matrix = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (nrows, ncols) = (matrix.len(), matrix[0].len());
    let mut map = HashMap::new();
    let mut loads = vec![];

    for idx in 0..COUNT {
        // NORTH
        for j in 0..ncols {
            let mut cnt = 0;
            for i in (0..nrows).rev() {
                match matrix[i][j] {
                    '#' => {
                        deposit_north(i + 1, j, &mut cnt, &mut matrix);
                    }
                    'O' => {
                        matrix[i][j] = '.';
                        cnt += 1;
                    }
                    _ => {}
                }
            }
            deposit_north(0, j, &mut cnt, &mut matrix);
        }
        // WEST
        for i in 0..nrows {
            let mut cnt = 0;
            for j in (0..ncols).rev() {
                match matrix[i][j] {
                    '#' => {
                        deposit_west(j + 1, i, &mut cnt, &mut matrix);
                    }
                    'O' => {
                        matrix[i][j] = '.';
                        cnt += 1;
                    }
                    _ => {}
                }
            }
            deposit_west(0, i, &mut cnt, &mut matrix);
        }
        // SOUTH
        for j in 0..ncols {
            let mut cnt = 0;
            for i in 0..nrows {
                match matrix[i][j] {
                    '#' => {
                        if cnt > 0 {
                            deposit_south(i - 1, j, &mut cnt, &mut matrix);
                            assert!(cnt == 0);
                        }
                    }
                    'O' => {
                        matrix[i][j] = '.';
                        cnt += 1;
                    }
                    _ => {}
                }
            }
            deposit_south(nrows - 1, j, &mut cnt, &mut matrix);
        }
        // EAST
        for i in 0..nrows {
            let mut cnt = 0;
            for j in 0..ncols {
                match matrix[i][j] {
                    '#' => {
                        if cnt > 0 {
                            deposit_east(j - 1, i, &mut cnt, &mut matrix);
                        }
                    }
                    'O' => {
                        matrix[i][j] = '.';
                        cnt += 1;
                    }
                    _ => {}
                }
            }
            deposit_east(ncols - 1, i, &mut cnt, &mut matrix);
        }
        // calc
        let (load, state) = load_and_state_of(&matrix);
        if let Some(i) = map.get(&state) {
            let period = idx - i;
            println!("the answer = {}", loads[i + (COUNT - i - 1) % period]);
            return;
        }
        map.insert(state, idx);
        loads.push(load);
    }
}
fn load_and_state_of(matrix: &[Vec<char>]) -> (usize, Vec<usize>) {
    let (nrows, ncols) = (matrix.len(), matrix[0].len());
    let mut state = vec![];
    let mut load = 0;
    for i in 0..nrows {
        let mut cnt = 0;
        for j in 0..ncols {
            match matrix[i][j] {
                'O' => {
                    cnt += 1;
                    load += nrows - i;
                }
                '#' => {
                    state.push(cnt);
                    cnt = 0;
                }
                _ => {}
            }
        }
        state.push(cnt);
    }
    (load, state)
}

fn deposit_north(mut k: usize, j: usize, cnt: &mut usize, matrix: &mut [Vec<char>]) {
    while *cnt > 0 {
        matrix[k][j] = 'O';
        *cnt -= 1;
        k += 1;
    }
}
fn deposit_west(mut k: usize, i: usize, cnt: &mut usize, matrix: &mut [Vec<char>]) {
    while *cnt > 0 {
        matrix[i][k] = 'O';
        *cnt -= 1;
        k += 1;
    }
}
fn deposit_south(mut k: usize, j: usize, cnt: &mut usize, matrix: &mut [Vec<char>]) {
    while *cnt > 0 {
        matrix[k][j] = 'O';
        *cnt -= 1;
        if *cnt > 0 {
            assert!(k > 0);
            k -= 1;
        }
    }
}
fn deposit_east(mut k: usize, i: usize, cnt: &mut usize, matrix: &mut [Vec<char>]) {
    while *cnt > 0 {
        matrix[i][k] = 'O';
        *cnt -= 1;
        if *cnt > 0 {
            assert!(k > 0);
            k -= 1;
        }
    }
}
