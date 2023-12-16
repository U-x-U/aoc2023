use std::collections::HashSet;
fn main() {
    let matrix = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (nrows, ncols) = (matrix.len(), matrix[0].len());
    let mut ans = 0;
    for i in 0..nrows {
        ans = ans.max(start_at(i, 0, 0, 1, &matrix));
        ans = ans.max(start_at(i, ncols - 1, 0, -1, &matrix));
    }
    for j in 0..ncols {
        ans = ans.max(start_at(0, j, 1, 0, &matrix));
        ans = ans.max(start_at(nrows - 1, j, -1, 0, &matrix));
    }
    println!("{}", ans);
}

fn start_at(st_i: usize, st_j: usize, st_di: isize, st_dj: isize, matrix: &[Vec<char>]) -> usize {
    let (nrows, ncols) = (matrix.len(), matrix[0].len());
    let mut seen_dir = HashSet::new();
    // let mut visited = HashSet::new();
    let mut q = vec![(st_i, st_j, st_di, st_dj)];
    let travel =
        |x: usize, y: usize, dx: isize, dy: isize, nq: &mut Vec<(usize, usize, isize, isize)>| {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            if nx < nrows && ny < ncols {
                nq.push((nx, ny, dx, dy));
            }
        };
    while !q.is_empty() {
        let mut nq = vec![];
        for (x, y, dx, dy) in q {
            if seen_dir.contains(&(x, y, dx, dy)) {
                continue;
            }
            seen_dir.insert((x, y, dx, dy));
            match matrix[x][y] {
                '.' => travel(x, y, dx, dy, &mut nq),
                '/' => match (dx, dy) {
                    (1, 0) => travel(x, y, 0, -1, &mut nq),
                    (0, 1) => travel(x, y, -1, 0, &mut nq),
                    (-1, 0) => travel(x, y, 0, 1, &mut nq),
                    (0, -1) => travel(x, y, 1, 0, &mut nq),
                    _ => unreachable!(),
                },
                '\\' => match (dx, dy) {
                    (1, 0) => travel(x, y, 0, 1, &mut nq),
                    (0, 1) => travel(x, y, 1, 0, &mut nq),
                    (-1, 0) => travel(x, y, 0, -1, &mut nq),
                    (0, -1) => travel(x, y, -1, 0, &mut nq),
                    _ => unreachable!(),
                },
                '-' => match (dx, dy) {
                    (0, 1) | (0, -1) => travel(x, y, dx, dy, &mut nq),
                    (1, 0) | (-1, 0) => {
                        travel(x, y, 0, 1, &mut nq);
                        travel(x, y, 0, -1, &mut nq);
                    }
                    _ => unreachable!(),
                },
                '|' => match (dx, dy) {
                    (1, 0) | (-1, 0) => travel(x, y, dx, dy, &mut nq),
                    (0, 1) | (0, -1) => {
                        travel(x, y, 1, 0, &mut nq);
                        travel(x, y, -1, 0, &mut nq);
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
        }
        q = nq;
    }
    seen_dir
        .into_iter()
        .map(|(a, b, _, _)| (a, b))
        .collect::<HashSet<_>>()
        .len()
}
