fn main() {
    let ans = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.bytes().fold(0, |acc, ch| (acc + ch as usize) * 17 % 256))
        .sum::<usize>();
    println!("{}", ans);
}
