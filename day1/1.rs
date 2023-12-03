fn main() {
    let input = include_str!("./input.txt");
    let ans = input.lines().fold(0, |acc, word| {
        acc + {
            word.chars()
                .find(|c| c.is_numeric())
                .and_then(|c| c.to_digit(10))
                .expect("digit no found")
                * 10
                + word
                    .chars()
                    .rev()
                    .find(|c| c.is_numeric())
                    .and_then(|c| c.to_digit(10))
                    .expect("digit no found")
        }
    });
    println!("{}", ans);
}
