fn main() {
    let matrix = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;
    let (nrows, ncols) = (matrix.len(), matrix[0].len());
    let get_num = |i: usize, j: usize| {
        if nrows <= i || ncols <= j || !matrix[i][j].is_numeric() {
            return None;
        }
        let mut idx = j;
        let mut ans = 0;
        while idx < ncols && matrix[i][idx].is_numeric() {
            idx = idx.wrapping_sub(1);
        }
        idx = idx.wrapping_add(1);

        while idx < ncols && matrix[i][idx].is_numeric() {
            ans = ans * 10 + matrix[i][idx].to_digit(10).expect("failed to parse digit");
            idx += 1;
        }
        Some(ans)
    };

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] != '*' {
                continue;
            }
            let mut nums = vec![];
            if i != 0 {
                // check numbers in the row above
                if matrix[i.wrapping_sub(1)][j].is_numeric() {
                    // one and only one
                    nums.push(
                        get_num(i.wrapping_sub(1), j)
                            .expect("failed to parse the number above the asterisk"),
                    );
                } else {
                    // two positions are possible
                    if let Some(num) = get_num(i.wrapping_sub(1), j - 1) {
                        nums.push(num);
                    }
                    if let Some(num) = get_num(i.wrapping_sub(1), j + 1) {
                        nums.push(num);
                    }
                }
            }
            // check numbers in the same row
            if let Some(num) = get_num(i, j.wrapping_sub(1)) {
                nums.push(num);
            }
            if let Some(num) = get_num(i, j + 1) {
                nums.push(num);
            }
            // check numbers in the row below
            if i != nrows - 1 {
                // check numbers in the row above
                if matrix[i + 1][j].is_numeric() {
                    // one and only one
                    nums.push(
                        get_num(i + 1, j).expect("failed to parse the number above the asterisk"),
                    );
                } else {
                    // two positions are possible
                    if let Some(num) = get_num(i + 1, j - 1) {
                        nums.push(num);
                    }
                    if let Some(num) = get_num(i + 1, j + 1) {
                        nums.push(num);
                    }
                }
            }
            // check the # and do calc
            if nums.len() == 2 {
                ans += nums[0] * nums[1];
            }
        }
    }
    println!("{}", ans);
}
