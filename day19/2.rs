use std::collections::HashMap;
const MAX_VAL: u32 = 4000;
fn main() {
    let workflows = include_str!("input.txt")
        .lines()
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
    // each label only have at most one prev label
    let mut prev: HashMap<&str, &str> = HashMap::new();
    for (&label, rules) in workflows.iter() {
        for rule in rules.iter() {
            let nlabel = get_next_label(rule);
            if nlabel == "A" || nlabel == "R" {
                continue;
            }
            prev.insert(nlabel, label);
        }
    }
    let mut ans = 0;
    for (&(mut label), rules) in workflows.iter() {
        // consider the case that some part is ACCEPTed in this workflow
        // CONTINUE if this workflow is impossible to lead to an ACCEPT
        if rules.iter().all(|rule| rule.find('A').is_none()) {
            continue;
        }
        let mut ranges: Vec<HashMap<&str, (u32, u32)>> = vec![];
        {
            let mut cur = vec![
                ("x", (0, MAX_VAL + 1)),
                ("m", (0, MAX_VAL + 1)),
                ("a", (0, MAX_VAL + 1)),
                ("s", (0, MAX_VAL + 1)),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>();
            for rule in rules {
                if rule.find('A').is_some() {
                    // this one matches
                    let mut matched = cur.clone();
                    do_match(rule, &mut matched);
                    ranges.push(matched);
                }
                // all the rules before this one does not match
                do_not_match(rule, &mut cur);
            }
        }

        while label != "in" {
            // if it's an unreachable case
            if !prev.contains_key(label) {
                ranges.clear();
                break;
            }
            let last_label = label.clone();
            label = prev[label];
            // all the rules in this workflow until last label does not match
            let mut nranges = vec![];
            for mut cur in ranges {
                for rule in workflows[label].iter() {
                    let nlabel = get_next_label(rule);
                    if nlabel == last_label {
                        do_match(rule, &mut cur);
                        break;
                    } else {
                        do_not_match(rule, &mut cur);
                    }
                }
                if cur.iter().all(|(_, (lo, hi))| hi > &(lo + 1)) {
                    nranges.push(cur);
                }
            }
            ranges = nranges;
        }
        for rg in ranges {
            ans += rg.into_iter().fold(1u128, |acc, (_, (lo, hi))| {
                acc * (hi.checked_sub(lo + 1).unwrap_or(0) as u128)
            });
        }
    }
    println!("{}", ans);
}

fn do_match<'a>(rule: &'a str, cur: &mut HashMap<&'a str, (u32, u32)>) {
    if let Some((ne, _)) = rule.split_once(':') {
        if let Some((cat, rate)) = ne.split_once('<') {
            cur.entry(&cat).and_modify(|rating| {
                rating.1 = rating
                    .1
                    .min(rate.parse::<u32>().expect("failed to parse rating"))
            });
        } else if let Some((cat, rate)) = ne.split_once('>') {
            cur.entry(&cat).and_modify(|rating| {
                rating.0 = rating
                    .0
                    .max(rate.parse::<u32>().expect("failed to parse rating"))
            });
        } else {
            unreachable!()
        }
    }
}
fn do_not_match<'a>(rule: &'a str, cur: &mut HashMap<&'a str, (u32, u32)>) {
    if let Some((ne, _)) = rule.split_once(':') {
        if let Some((cat, rate)) = ne.split_once('<') {
            cur.entry(&cat).and_modify(|rating| {
                rating.0 = rating
                    .0
                    .max(rate.parse::<u32>().expect("failed to parse rating") - 1)
            });
        } else if let Some((cat, rate)) = ne.split_once('>') {
            cur.entry(&cat).and_modify(|rating| {
                rating.1 = rating
                    .1
                    .min(rate.parse::<u32>().expect("failed to parse rating") + 1)
            });
        } else {
            unreachable!()
        }
    }
}
fn get_next_label(rule: &str) -> &str {
    if let Some((_, nlabel)) = rule.split_once(':') {
        nlabel
    } else {
        rule
    }
}
