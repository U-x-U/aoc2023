// Q: for case twone, is the decrypted number 21?
fn main() {
    let input = include_str!("./input.txt");
    let spelled_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let ans = input.lines().fold(0, |acc, word| {
        acc + {
            // we know that these digits are guaranteed to appear
            // b.c. the input is the same as part1
            let digit1_idx = word
                .find(char::is_numeric)
                .expect("numeric digit not found");
            let digit2_idx = word
                .rfind(char::is_numeric)
                .expect("numeric digit not found");

            let mut d1: (usize, i32) = (
                digit1_idx,
                word.chars()
                    .nth(digit1_idx)
                    .and_then(|ch| ch.to_digit(10))
                    .expect("failed to parse digit") as i32,
            );
            let mut d2: (usize, i32) = (
                digit2_idx,
                word.chars()
                    .nth(digit2_idx)
                    .and_then(|ch| ch.to_digit(10))
                    .expect("failed to parse digit") as i32,
            );
            for (i, spelled) in spelled_digits.iter().enumerate() {
                let target_digit = i as i32 + 1;
                if let Some(idx) = word.find(spelled) {
                    d1 = d1.min((idx, target_digit));
                }
                if let Some(idx) = word.rfind(spelled) {
                    d2 = d2.max((idx, target_digit));
                }
            }
            d1.1 * 10 + d2.1
        }
    });
    println!("{}", ans);
}
