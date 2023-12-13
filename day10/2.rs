// the enclosed area in on the right along the clockwise traversal of the loop
// how to traverse clockwisely?
// find any leftmost cell in the loop.
// from this cell, go up or go right is the direction of "clockwise"
// KNOWN ISSUE:
//   if S is the only leftmost cell.
//   however we may not know which direction does S goes to (maybe RIGHT|UP|DOWN are all possible
//   by looking at these three adjacent cells alone. we can know this during the DFS, but it's not easy to code)
//   To fix this, we can find the rightmost cell instead (or just do d left-right flip) in this case
use std::collections::HashSet;
use std::collections::VecDeque;
fn main() {
    let matrix = include_str!("input.txt")
        // let matrix = include_str!("test.txt")
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (nrows, ncols) = (matrix.len(), matrix[0].len());
    let mut loop_cells = HashSet::new();
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
            let len = dfs(cur, (s_i, s_j), (s_i, s_j), &matrix, &mut loop_cells);
            (len != 0).then(|| len)
        })
        .expect("failed to find a valid start direction");
    assert!(loop_len == loop_cells.len());

    let leftmost = loop_cells
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .cloned()
        .expect("failed to find the leftmost cell");
    assert_ne!(
        matrix[leftmost.0][leftmost.1], 'S',
        "the leftmost cell is S. find more info in beginning comment"
    );
    let mut dq = VecDeque::new();
    let dir = {
        match matrix[leftmost.0][leftmost.1] {
            '|' | 'L' => (-1, 0),
            'F' => (0, 1),
            _ => unreachable!(),
        }
    };
    dfs1(leftmost, dir, leftmost, &matrix, &mut dq, &loop_cells);
    let mut matrix = matrix;
    let mut cnt = 0;
    while let Some((i, j)) = dq.pop_front() {
        if matrix[i][j] == 'I' {
            continue;
        }
        cnt += 1;
        matrix[i][j] = 'I';
        for (ni, nj) in [
            (i, j + 1),
            (i.wrapping_sub(1), j),
            (i, j.wrapping_sub(1)),
            (i + 1, j),
        ] {
            if ni >= nrows || nj >= ncols || loop_cells.contains(&(ni, nj)) || matrix[ni][nj] == 'I'
            {
                continue;
            }
            dq.push_back((ni, nj));
        }
    }

    println!("in cnt = {}", cnt);
    println!("loop len = {}", loop_len);
}

fn possible_dir(ch: char) -> Vec<(isize, isize)> {
    match ch {
        '.' => vec![],
        '|' => vec![(-1, 0), (1, 0)],  // up and down
        '-' => vec![(0, -1), (0, 1)],  // left and right
        'L' => vec![(-1, 0), (0, 1)],  // up and right
        'J' => vec![(-1, 0), (0, -1)], // up and left
        '7' => vec![(1, 0), (0, -1)],  // down and left
        'F' => vec![(1, 0), (0, 1)],   // down and right
        'S' => vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
        _ => unreachable!(),
    }
}
fn next_possible_pos((i, j): (usize, usize), matrix: &[Vec<char>]) -> Vec<(usize, usize)> {
    possible_dir(matrix[i][j])
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
    loop_cells: &mut HashSet<(usize, usize)>,
) -> usize {
    let (i, j) = cur;
    if i >= matrix.len() || j >= matrix[0].len() {
        return 0;
    }
    if cur == start {
        loop_cells.insert(cur);
        return 1;
    }
    let next_v = next_possible_pos(cur, matrix);
    if next_v.iter().all(|&pos| pos != pre) {
        return 0;
    } else {
        let nxt = *next_v.iter().find(|&&pos| pos != pre).expect("not found");
        let len = dfs(nxt, cur, start, &matrix, loop_cells);
        if len != 0 {
            loop_cells.insert(cur);
            return len + 1;
        }
    }
    return 0;
}

fn get_right_dir(i: isize, j: isize) -> (isize, isize) {
    match (i, j) {
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        _ => unreachable!(),
    }
}
fn dfs1(
    cur: (usize, usize),
    dir: (isize, isize),
    dst: (usize, usize),
    matrix: &[Vec<char>],
    dq: &mut VecDeque<(usize, usize)>,
    loop_cells: &HashSet<(usize, usize)>,
) {
    let (i, j) = cur;
    assert!(i < matrix.len() && j < matrix[0].len() && loop_cells.contains(&(i, j)));
    let nxt = (i.wrapping_add_signed(dir.0), j.wrapping_add_signed(dir.1));
    assert!(nxt.0 < matrix.len() && nxt.1 < matrix[0].len() && loop_cells.contains(&(i, j)));
    let (ni, nj) = nxt;
    // should check the right-hand side of BOTH cur and nxt
    {
        let (di, dj) = get_right_dir(dir.0, dir.1);
        let (adj_i, adj_j) = (ni.wrapping_add_signed(di), nj.wrapping_add_signed(dj));
        if adj_i < matrix.len() && adj_j < matrix[0].len() && !loop_cells.contains(&(adj_i, adj_j))
        {
            dq.push_back((adj_i, adj_j));
        }
        let (adj_i, adj_j) = (i.wrapping_add_signed(di), j.wrapping_add_signed(dj));
        if adj_i < matrix.len() && adj_j < matrix[0].len() && !loop_cells.contains(&(adj_i, adj_j))
        {
            dq.push_back((adj_i, adj_j));
        }
    }

    if nxt == dst {
        return;
    }
    let ndir: (isize, isize) = possible_dir(matrix[nxt.0][nxt.1])
        .into_iter()
        .find(|&ndir| ndir.0 + dir.0 != 0 || ndir.1 + dir.1 != 0)
        .expect("failed to find next dir");
    dfs1(nxt, ndir, dst, matrix, dq, loop_cells);
}
