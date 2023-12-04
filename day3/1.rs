// impl char {
//     fn is_symbol(&self) -> bool {
//         !self.is_numeric() && self != '.'
//     }
// }
fn main() {
    let matrix = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;

    for i in 0..matrix.len() {
        let row: &Vec<char> = &matrix[i];
        let mut cur = 0;
        let mut is_part_number = false;
        for j in 0..row.len() {
            let ch = row[j];
            if ch.is_numeric() {
                let d = ch.to_digit(10).expect("failed to parse digit");
                if j != 0 && row[j - 1].is_numeric() {
                    // not initial of a number
                    cur = cur * 10 + d;
                } else {
                    // initial of a number
                    cur = d;
                    if j != 0 {
                        // check the triplet in the previous col if it's not the first col
                        is_part_number |= matrix[i][j - 1] != '.'
                            || (i != 0
                                && !matrix[i - 1][j - 1].is_numeric()
                                && matrix[i - 1][j - 1] != '.')
                            || (i != matrix.len() - 1
                                && !matrix[i + 1][j - 1].is_numeric()
                                && matrix[i + 1][j - 1] != '.');
                    }
                }
                // check the char above and below.
                is_part_number |=
                    (i != 0 && !matrix[i - 1][j].is_numeric() && matrix[i - 1][j] != '.')
                        || (i != matrix.len() - 1
                            && !matrix[i + 1][j].is_numeric()
                            && matrix[i + 1][j] != '.');
            } else {
                if cur != 0 {
                    is_part_number |= matrix[i][j] != '.'
                        || (i != 0 && !matrix[i - 1][j].is_numeric() && matrix[i - 1][j] != '.')
                        || (i != matrix.len() - 1
                            && !matrix[i + 1][j].is_numeric()
                            && matrix[i + 1][j] != '.');
                    // add if is part number
                    if is_part_number {
                        ans += cur;
                    }
                    cur = 0;
                }
                is_part_number = false;
            }
        }
        if cur != 0 && is_part_number {
            ans += cur;
        }
    }
    println!("{}", ans);
}
