use std::collections::HashMap;
fn main() {
    let input = include_str!("input.txt");
    let ans = input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').expect("failed to split at space once");
            let springs = springs.chars().collect::<Vec<_>>();
            let groups = groups
                .split(',')
                .map(|num_str| num_str.parse::<usize>().expect("failed to parse num"))
                .collect::<Vec<_>>();
            let mut memo = HashMap::new();
            let ans = dfs(&springs, &groups, 0, 0, 0, &mut memo);
            ans
        })
        .fold(0, |acc, x| {
            println!("{}", x);
            acc + x
        });

    println!("{}", ans);
}

fn dfs(
    springs: &Vec<char>,
    groups: &Vec<usize>,
    i: usize,
    j: usize,
    k: usize,
    memo: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if i == springs.len() && j == groups.len() {
        if k == 0 {
            return 1;
        }
        return 0;
    }
    if i == springs.len() {
        assert!(j < groups.len());
        if k != 0 && k == groups[j] {
            return dfs(springs, groups, i, j + 1, 0, memo);
        }
        return 0;
    }
    if j == groups.len() {
        if k == 0 && springs.iter().skip(i).all(|&ch| ch != '#') {
            return 1;
        }
        return 0;
    }
    match springs[i] {
        '.' => {
            if k == 0 {
                dfs(springs, groups, i + 1, j, 0, memo)
            } else {
                if k == groups[j] {
                    dfs(springs, groups, i + 1, j + 1, 0, memo)
                } else {
                    0
                }
            }
        }
        '#' => dfs(springs, groups, i + 1, j, k + 1, memo),
        '?' => {
            if let Some(&ans) = memo.get(&(i, j, k)) {
                return ans;
            }
            // ? can serve as a # or a .
            let ans = { dfs(springs, groups, i + 1, j, k + 1, memo) } + {
                if k == 0 {
                    dfs(springs, groups, i + 1, j, 0, memo)
                } else {
                    if k == groups[j] {
                        dfs(springs, groups, i + 1, j + 1, 0, memo)
                    } else {
                        0
                    }
                }
            };
            memo.insert((i, j, k), ans);
            ans
        }
        _ => unreachable!(),
    }
}
