use std::collections::HashMap;
fn main() {
    let mut input_it = include_str!("input.txt").lines();
    let workflows = input_it
        .by_ref()
        .map_while(|line| {
            if line.is_empty() {
                return None;
            }
            let (label, rules) = line.split_once('{').expect("failed to split once at {");
            let rules = rules
                .strip_suffix('}')
                .expect("failed to strip suffix }")
                .split(',')
                .collect::<Vec<_>>();
            Some((label, rules))
        })
        .collect::<HashMap<_, _>>();
    let ans = input_it
        .map(|line| {
            let line = line.trim_matches('{');
            let line = line.trim_matches('}');
            line.split(',')
                .map(|rating| {
                    let (cat, rate) = rating.split_once('=').expect("failed to split once at =");
                    let rate = rate.parse::<u32>().expect("failed to parse rate");
                    (cat, rate)
                })
                .collect::<HashMap<&str, u32>>()
        })
        .filter_map(|part| {
            let mut label = "in";
            loop {
                for &rule in workflows[label].iter() {
                    if let Some((ne, nlabel)) = rule.split_once(':') {
                        if let Some((cat, rate)) = ne.split_once('<') {
                            if part[cat] < rate.parse::<u32>().expect("failed to parse rating") {
                                label = nlabel;
                                break;
                            }
                        } else {
                            let (cat, rate) =
                                ne.split_once('>').expect("neither < nor > found in ne");
                            if part[cat] > rate.parse::<u32>().expect("failed to parse rating") {
                                label = nlabel;
                                break;
                            }
                        }
                    } else {
                        label = rule;
                    }
                }
                match label {
                    "A" => return Some(part.into_iter().fold(0, |acc, (_, rate)| acc + rate)),
                    "R" => return None,
                    _ => {}
                }
            }
        })
        .sum::<u32>();
    println!("{}", ans);
}
