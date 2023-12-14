fn main() {
    let matrix = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (nrows, ncols) = (matrix.len(), matrix[0].len());
    let mut ans = 0;
    for j in 0..ncols {
        let mut cnt = 0;
        for i in (0..nrows).rev() {
            match matrix[i][j] {
                '#' => {
                    let mut load = nrows - i - 1;
                    while cnt > 0 {
                        ans += load;
                        cnt -= 1;
                        load -= 1;
                    }
                }
                'O' => {
                    cnt += 1;
                }
                _ => {}
            }
        }
        let mut load = nrows;
        while cnt > 0 {
            ans += load;
            cnt -= 1;
            load -= 1;
        }
    }
    println!("ans = {}", ans);
}
