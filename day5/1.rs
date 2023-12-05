fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut seeds = lines
        .next()
        .expect("seed line")
        .strip_prefix("seeds: ")
        .expect("strip prefix seeds")
        .split_whitespace()
        .map(|num_str| num_str.parse::<u64>().expect("failed to parse seed val"))
        .collect::<Vec<_>>();

    lines.next();
    for _i in 0..7 {
        lines.next(); // map title
        let mut map = lines
            .by_ref()
            .take_while(|&line| line.chars().count() > 0)
            .map(|line| {
                let mut vals = line
                    .split_whitespace()
                    .map(|num_str| num_str.parse::<u64>().expect("failed to parse s2s val"));
                let val = vals.next().expect("failed to get soil st");
                let key = vals.next().expect("failed to get seed st");
                let range = vals.next().expect("failed to get range");
                (key, val, range)
            })
            .collect::<Vec<(u64, u64, u64)>>();
        map.sort_unstable();
        seeds = seeds
            .into_iter()
            .map(|seed| {
                let i = map.partition_point(|&(k, _, _)| k < seed);
                if i == 0 {
                    return seed;
                }
                if let Some(&(k, v, r)) = map.get(i - 1) {
                    if seed < k + r {
                        return v + seed - k;
                    }
                }
                return seed;
            })
            .collect();
    }

    println!(
        "{}",
        seeds
            .into_iter()
            .min_by(|x, y| x.cmp(y))
            .expect("failed to find min")
    );
}
