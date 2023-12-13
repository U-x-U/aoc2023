fn main() {
    let matrix = include_str!("input.txt")
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (nrows, ncols) = (matrix.len(), matrix[0].len());
    let (s_i, s_j) = {
        let (mut i, mut j) = (0, 0);
        while i < nrows {
            while j < ncols {
                if matrix[i][j] == 'S' {
                    break;
                }
                j += 1;
            }
            if j != ncols {
                break;
            } else {
                j = 0;
            }
            i += 1;
        }
        assert!(i != nrows);
        (i, j)
    };
    let loop_len = next_possible_pos((s_i, s_j), &matrix)
        .into_iter()
        .find_map(|cur| {
            let len = dfs(cur, (s_i, s_j), (s_i, s_j), &matrix);
            (len != 0).then(|| len)
        })
        .expect("failed to find a valid start direction");
    println!("{}", loop_len / 2);
}

fn next_dir(ch: char) -> Vec<(isize, isize)> {
    match ch {
        '.' => vec![],
        '|' => vec![(-1, 0), (1, 0)],
        '-' => vec![(0, -1), (0, 1)],
        'L' => vec![(-1, 0), (0, 1)],
        'J' => vec![(-1, 0), (0, -1)],
        '7' => vec![(1, 0), (0, -1)],
        'F' => vec![(1, 0), (0, 1)],
        'S' => vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
        _ => unreachable!(),
    }
}
fn next_possible_pos((i, j): (usize, usize), matrix: &[Vec<char>]) -> Vec<(usize, usize)> {
    next_dir(matrix[i][j])
        .into_iter()
        .filter_map(|(di, dj)| {
            let (ni, nj) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
            (ni < matrix.len() && nj < matrix[0].len()).then(|| (ni, nj))
        })
        .collect::<Vec<_>>()
}
fn dfs(
    cur: (usize, usize),
    pre: (usize, usize),
    start: (usize, usize),
    matrix: &[Vec<char>],
) -> usize {
    assert!(cur.0 < matrix.len() && cur.1 < matrix[0].len());
    if cur == start {
        return 1;
    }
    let next_v = next_possible_pos(cur, &matrix);
    if next_v.iter().all(|&pos| pos != pre) {
        return 0;
    } else {
        let nxt = *next_v.iter().find(|&&pos| pos != pre).expect("not found");
        let len = dfs(nxt, cur, start, &matrix);
        if len != 0 {
            return len + 1;
        }
    }
    return 0;
}
