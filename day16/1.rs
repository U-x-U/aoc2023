use std::collections::HashSet;
fn main() {
    let matrix = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (nrows, ncols) = (matrix.len(), matrix[0].len());
    let mut seen = HashSet::new();
    let mut q = vec![(0, 0, 0, 1)];
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
            if seen.contains(&(x, y, dx, dy)) {
                continue;
            }
            seen.insert((x, y, dx, dy));
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
    println!(
        "{}",
        seen.into_iter()
            .map(|(x, y, _, _)| (x, y))
            .collect::<HashSet<(usize, usize)>>()
            .len()
    );
}
