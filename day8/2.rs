use std::collections::HashMap;
fn main() {
    let mut input_iter = include_str!("input.txt").lines();
    let instructions = input_iter
        .next()
        .expect("failed to get instr")
        .chars()
        .collect::<Vec<_>>();
    input_iter.next();
    let network = input_iter
        .map(|line| {
            let (mut node, mut lr) = line.split_once('=').expect("failed to split at =");
            node = node.trim();
            lr = lr.trim();
            {
                let pat: &[_] = &['(', ')'];
                lr = lr.trim_matches(pat);
            }
            let (lnode, mut rnode) = lr.split_once(',').expect("failed to split at ,");
            rnode = rnode.trim();
            (node, (lnode, rnode))
        })
        .collect::<HashMap<_, _>>();

    let start_nodes_cycle = network
        .keys()
        .cloned()
        .filter_map(|node| {
            if !node.ends_with('A') {
                return None;
            }
            let mut cur = node;
            let mut cycle_map = HashMap::new();
            let mut indices = vec![];
            let (cycle_start, period) = instructions
                .iter()
                .cloned()
                .cycle()
                .enumerate()
                .find_map(|(i, instr)| {
                    let idx = i % instructions.len();
                    if let Some(&j) = cycle_map.get(&(cur, idx)) {
                        return Some((j, i - j));
                    }
                    cycle_map.insert((cur, idx), i);
                    let (lnode, rnode) = network[cur];
                    cur = match instr {
                        'L' => lnode,
                        'R' => rnode,
                        _ => unreachable!(),
                    };
                    if cur.ends_with('Z') {
                        indices.push(i);
                    }
                    None
                })
                .expect("failed to find cycle pattern");
            Some((cycle_start, period, indices))
        })
        .collect::<Vec<_>>();
    println!("{:?}", start_nodes_cycle);
    // calc based on cycles and terminatable-indices
    // [(3, 18827, [18826]), (2, 17141, [17140]), (2, 20513, [20512]), (4, 19951, [19950]), (3, 12083, [12082]), (3, 22199, [22198])]
    println!(
        "ans = {}",
        start_nodes_cycle
            .into_iter()
            .fold(1, |acc, (_, period, _)| gcd(acc, period))
    );
}
use std::mem::swap;
fn gcd(x: usize, y: usize) -> usize {
    let (mut xx, mut yy) = (x, y);
    if xx < yy {
        swap(&mut xx, &mut yy);
    }
    while xx % yy != 0 {
        let temp = xx;
        xx = yy;
        yy = temp % yy;
    }
    x * y / yy
}
