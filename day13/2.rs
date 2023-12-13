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
                let mut diff_cnt = 0;
                while 0 <= a && b < nrows && diff_cnt <= 1 {
                    diff_cnt += graph[a as usize]
                        .iter()
                        .zip(graph[b].iter())
                        .map(|(x, y)| (x != y) as usize)
                        .sum::<usize>();
                    a -= 1;
                    b += 1;
                }
                if diff_cnt == 1 && (a < 0 || nrows <= b) {
                    return 100 * i;
                }
            }
            // i: how many cols is to left
            for i in 1..ncols {
                let (mut a, mut b) = (i as isize - 1, i);
                let mut diff_cnt = 0;
                while (0 <= a && b < ncols) && diff_cnt <= 1 {
                    diff_cnt += graph
                        .iter()
                        .map(|line| (line[a as usize] != line[b]) as usize)
                        .sum::<usize>();
                    a -= 1;
                    b += 1;
                }
                if diff_cnt == 1 && (a < 0 || ncols <= b) {
                    return i;
                }
            }
            unreachable!()
        })
        .sum::<usize>();
    println!("{:?}", ans);
}
