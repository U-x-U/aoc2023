fn main() {
    let mut lines = include_str!("input.txt").lines();
    let t = lines
        .by_ref()
        .next()
        .expect("time line")
        .strip_prefix("Time:")
        .expect("strip prefix")
        .chars()
        .filter(|ch| ch.is_numeric())
        .fold(0i128, |acc, ch| {
            acc * 10i128 + ch.to_digit(10).expect("parse digit") as i128
        }) as i128;
    let d = lines
        .next()
        .expect("dist line")
        .strip_prefix("Distance:")
        .expect("strip prefix")
        .chars()
        .filter(|ch| ch.is_numeric())
        .fold(0i128, |acc, ch| {
            acc * 10i128 + ch.to_digit(10).expect("parse digit") as i128
        }) as i128;

    let ans = {
        let delta_sq = t * t - 4 * d - 4;
        if delta_sq < 0 || delta_sq == 0 && t % 2 != 0 {
            0
        } else if delta_sq == 0 {
            1
        } else {
            let (t, d) = (t as f64, d as f64);
            let delta: f64 = (t * t - 4.0 * d - 4.0).sqrt();
            let lb = ((t - delta) / 2.0).ceil().max(0.0);
            let ub = ((t + delta) / 2.0).floor();
            // is it possible?
            if ub < lb {
                0
            } else {
                (ub - lb) as i128 + 1
            }
        }
    };
    println!("{}", ans);
}
