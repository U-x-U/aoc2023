use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let ans = input.lines().fold(0, |acc, card| {
        acc + {
            let (winning_nums_str, my_nums_str) =
                card.split_once('|').expect("failed to split at |");
            let (_, winning_nums_str) = winning_nums_str
                .split_once(':')
                .expect("failed to split at colon");
            let winning_nums = winning_nums_str
                .split_whitespace()
                .map(|num| num.parse::<u32>().expect("failed to parse int"))
                .collect::<HashSet<u32>>();
            let mut score = 1;
            for my_num in my_nums_str
                .split_whitespace()
                .map(|num| num.parse::<u32>().expect("failed to parse int"))
            {
                if winning_nums.contains(&my_num) {
                    score <<= 1;
                }
            }
            score / 2
        }
    });
    println!("{}", ans);
}
