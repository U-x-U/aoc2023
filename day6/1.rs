fn main() {
    let mut lines = include_str!("input.txt").lines();
    let time = lines
        .by_ref()
        .next()
        .expect("time line")
        .strip_prefix("Time:")
        .expect("strip prefix")
        .split_whitespace()
        .map(|t_str| t_str.parse::<i32>().expect("parse num"))
        .collect::<Vec<_>>();
    let dist = lines
        .next()
        .expect("dist line")
        .strip_prefix("Distance:")
        .expect("strip prefix")
        .split_whitespace()
        .map(|d_str| d_str.parse::<i32>().expect("parse num"))
        .collect::<Vec<_>>();

    assert!(time.len() == dist.len());
    let mut ans = 1;
    for i in 0..time.len() {
        let (t, d) = (time[i], dist[i]);
        let delta_sq = t * t - 4 * d - 4;
        if delta_sq < 0 || delta_sq == 0 && t % 2 != 0 {
            ans = 0;
            break;
        } else if delta_sq == 0 {
            ans = 1;
            continue;
        }
        let (t, d) = (t as f64, d as f64);
        let delta: f64 = (t * t - 4.0 * d - 4.0).sqrt();
        let lb = ((t - delta) / 2.0).ceil().max(0.0);
        let ub = ((t + delta) / 2.0).floor();
        // is it possible?
        if ub < lb {
            ans = 0;
            break;
        }
        ans *= ub as i32 - lb as i32 + 1;
    }
    println!("{}", ans);
}
