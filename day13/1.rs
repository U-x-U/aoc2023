// so is it guaranteed that each graph only have one mirror?
fn main() {
    let ans = include_str!("input.txt")
        .split("\n\n")
        .map(|s| {
            let graph = s
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let (nrows, ncols) = (graph.len(), graph[0].len());
            // i: how many rows is above the line of reflection
            for i in 1..nrows {
                let (mut a, mut b) = (i as isize - 1, i);
                while 0 <= a && b < nrows {
                    if graph[a as usize] != graph[b] {
                        break;
                    }
                    a -= 1;
                    b += 1;
                }
                if a < 0 || nrows <= b {
                    return 100 * i;
                }
            }
            // i: how many cols is to left
            for i in 1..ncols {
                let (mut a, mut b) = (i as isize - 1, i);
                while 0 <= a && b < ncols {
                    if graph.iter().any(|line| line[a as usize] != line[b]) {
                        break;
                    }
                    a -= 1;
                    b += 1;
                }
                if a < 0 || ncols <= b {
                    return i;
                }
            }
            unreachable!()
        })
        .sum::<usize>();
    println!("{:?}", ans);
}
