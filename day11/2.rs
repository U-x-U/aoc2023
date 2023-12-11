const EXPAND_MUL: isize = 1_000_000;
fn main() {
    let image = include_str!("input.txt")
        // let image = include_str!("test.txt")
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let empty_row_indices = (0..image.len())
        .filter_map(|row_idx| {
            if image[row_idx].iter().all(|&cell| cell == '.') {
                Some(row_idx as isize)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let empty_col_indices = (0..image[0].len())
        .filter_map(|col_idx| {
            if image.iter().all(|row| row[col_idx] == '.') {
                Some(col_idx as isize)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut galaxies = vec![];
    for i in 0..image.len() {
        for j in 0..image[i].len() {
            if image[i][j] == '#' {
                galaxies.push((i as isize, j as isize));
            }
        }
    }
    let mut ans = 0;
    let calc_dist = |(x1, y1): (isize, isize), (x2, y2): (isize, isize)| -> isize {
        (x1 - x2).abs() + (y1 - y2).abs()
    };
    let count_empty_rows = |x1: isize, x2: isize| -> isize {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        empty_row_indices.partition_point(|&x| x < x2) as isize
            - empty_row_indices.partition_point(|&x| x < x1) as isize
    };
    let count_empty_cols = |y1: isize, y2: isize| -> isize {
        let (y1, y2) = (y1.min(y2), y1.max(y2));
        empty_col_indices.partition_point(|&y| y < y2) as isize
            - empty_col_indices.partition_point(|&y| y < y1) as isize
    };
    for i in 0..galaxies.len() {
        for j in 0..i {
            ans += calc_dist(galaxies[i], galaxies[j])
                + (EXPAND_MUL - 1)
                    * (count_empty_rows(galaxies[i].0, galaxies[j].0)
                        + count_empty_cols(galaxies[i].1, galaxies[j].1));
        }
    }
    println!("{}", ans);
}
