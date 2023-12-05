fn main() {
    // let mut lines = include_str!("input.txt").lines();
    let mut lines = include_str!("test.txt").lines();
    let mut seeds_iter = lines
        .next()
        .expect("seed line")
        .strip_prefix("seeds: ")
        .expect("strip prefix seeds")
        .split_whitespace()
        .map(|num_str| num_str.parse::<u64>().expect("failed to parse seed val"));
    let mut seeds: Vec<(u64, u64)> = vec![];
    while let (Some(st), Some(rg)) = (seeds_iter.next(), seeds_iter.next()) {
        seeds.push((st, st + rg));
    }

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
        seeds = seeds.into_iter().fold(vec![], |mut acc, (lo, hi)| {
            // map (lo, hi) to [(?, ?), (?, ?), ...]
            let mut cur = lo;
            for &(key, val, range) in map.iter() {
                // key..key + range
                if cur >= hi {
                    // finished range lo..hi
                    break;
                }
                if hi <= key {
                    // cur..hi does not need remap
                    acc.push((cur, hi));
                    break;
                }
                if cur < key {
                    // cur..key does not need remap
                    acc.push((cur, key));
                    // key..min(hi, key + range)
                    let nxt = hi.min(key + range);
                    acc.push((val, nxt - key + val));
                    cur = nxt;
                } else if cur < key + range {
                    // cur..min(hi, key + range)
                    let nxt = hi.min(key + range);
                    acc.push((cur - key + val, nxt - key + val));
                    cur = nxt;
                } else {
                    continue;
                }
            }
            if cur < hi {
                acc.push((cur, hi));
            }
            return acc;
        });
    }
    seeds.sort_unstable();
    seeds.iter().for_each(|&seed| println!("{:?}", seed));
    println!("{}", seeds.first().expect("failed to get first of seeds").0);
}
