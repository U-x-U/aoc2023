use std::collections::HashSet;
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
    let mut q = vec![(s_i, s_j)];
    let mut seen = HashSet::new();
    seen.insert(s_i * ncols + s_j);
    let mut d = 0;
    let next_possible_pos = |i: usize, j: usize| -> Vec<(usize, usize)> {
        match matrix[i][j] {
            '.' => vec![],
            '|' => vec![(i.wrapping_sub(1), j), (i + 1, j)],
            '-' => vec![(i, j.wrapping_sub(1)), (i, j + 1)],
            'L' => vec![(i.wrapping_sub(1), j), (i, j + 1)],
            'J' => vec![(i.wrapping_sub(1), j), (i, j.wrapping_sub(1))],
            '7' => vec![(i + 1, j), (i, j.wrapping_sub(1))],
            'F' => vec![(i + 1, j), (i, j + 1)],
            'S' => vec![
                (i + 1, j),
                (i, j + 1),
                (i.wrapping_sub(1), j),
                (i, j.wrapping_sub(1)),
            ],
            _ => unreachable!(),
        }
    };
    let is_match = |(i1, j1): (usize, usize), (i2, j2): (usize, usize)| -> bool {
        let nxt = next_possible_pos(i2, j2);
        nxt.iter().any(|p| p == &(i1, j1))
    };
    while !q.is_empty() {
        let mut nq = vec![];
        for (i, j) in q {
            for (ni, nj) in next_possible_pos(i, j) {
                if ni >= nrows
                    || nj >= ncols
                    || seen.contains(&(ni * ncols + nj))
                    || !is_match((i, j), (ni, nj))
                {
                    continue;
                }
                seen.insert(ni * ncols + nj);
                nq.push((ni, nj));
            }
        }
        d += 1;
        if nq.len() == 1 {
            println!("{}", d);
            break;
        } else if nq.len() == 2 && nq[0] == nq[1] {
            println!("{}", d + 1);
            break;
        }
        q = nq;
    }
}
