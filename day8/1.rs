use std::collections::HashMap;
fn main() {
    let mut input_iter = include_str!("input.txt").lines();
    // let mut input_iter = include_str!("test.txt").lines();
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
    let mut cur = "AAA";
    for i in 0.. {
        let (lnode, rnode) = network[cur];
        cur = match instructions[i % instructions.len()] {
            'L' => lnode,
            'R' => rnode,
            _ => unreachable!(),
        };
        if cur == "ZZZ" {
            println!("{}", i + 1);
            break;
        }
    }
}
