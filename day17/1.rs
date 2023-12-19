use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
fn main() {
    let matrix = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).expect("failed to parse digit"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (nrows, ncols) = (matrix.len(), matrix[0].len());
    let mut heap = BinaryHeap::new();
    let mut set = HashSet::new();
    heap.push(Reverse((0, 0, 0, 0, 0, 1)));

    while let Some(Reverse((mut cur, x, y, dx, dy, cnt))) = heap.pop() {
        cur += matrix[x][y];
        if x == nrows - 1 && y == ncols - 1 {
            println!("{}", cur - matrix[0][0]);
            break;
        }
        if set.contains(&(x, y, dx, dy, cnt)) {
            continue;
        }
        set.insert((x, y, dx, dy, cnt));
        for (ndx, ndy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (x.wrapping_add_signed(ndx), y.wrapping_add_signed(ndy));
            let ncnt = if ndx == dx && ndy == dy { cnt + 1 } else { 1 };
            if ncnt <= 3
                && (ndx + dx != 0 || ndy + dy != 0)
                && nx < nrows
                && ny < ncols
                && !set.contains(&(nx, ny, ndx, ndy, ncnt))
            {
                heap.push(Reverse((cur, nx, ny, ndx, ndy, ncnt)));
            }
        }
    }
}
