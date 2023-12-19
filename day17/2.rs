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

    while let Some(Reverse((cur, x, y, dx, dy, cnt))) = heap.pop() {
        if x == nrows - 1 && y == ncols - 1 {
            println!("{}", cur);
            break;
        }
        if set.contains(&(x, y, dx, dy, cnt)) {
            continue;
        }
        set.insert((x, y, dx, dy, cnt));
        for (ndx, ndy) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (mut nx, mut ny, mut ncur, mut ncnt) = (x, y, cur, cnt);
            let mut move_once = || {
                nx = nx.wrapping_add_signed(ndx);
                ny = ny.wrapping_add_signed(ndy);
                if nx >= nrows || ny >= ncols {
                    return true;
                }
                ncur += matrix[nx][ny];
                false
            };
            if ndx + dx == 0 && ndy + dy == 0 {
                continue;
            }
            if ndx == dx && ndy == dy {
                ncnt += 1;
                if move_once() {
                    continue;
                }
            } else {
                if (0..4).any(|_| move_once()) {
                    continue;
                }
                ncnt = 4;
            }

            if ncnt <= 10 && !set.contains(&(nx, ny, ndx, ndy, ncnt)) {
                heap.push(Reverse((ncur, nx, ny, ndx, ndy, ncnt)));
            }
        }
    }
}
