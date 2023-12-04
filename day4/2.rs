use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let mut card_cnts = vec![1; input.lines().count()];
    input.lines().enumerate().for_each(|(idx, card)| {
        let (winning_nums_str, my_nums_str) = card.split_once('|').expect("failed to split at |");
        let (_, winning_nums_str) = winning_nums_str
            .split_once(':')
            .expect("failed to split at colon");
        let winning_nums = winning_nums_str
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("failed to parse int"))
            .collect::<HashSet<u32>>();
        let mut cnt = 0;
        for my_num in my_nums_str
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("failed to parse int"))
        {
            if winning_nums.contains(&my_num) {
                cnt += 1;
            }
        }
        for i in idx + 1..=idx + cnt {
            card_cnts[i] += card_cnts[idx];
        }
    });
    println!("{}", card_cnts.into_iter().sum::<u32>());
}
