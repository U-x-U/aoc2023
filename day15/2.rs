fn main() {
    let mut boxes = vec![vec![]; 256];
    include_str!("input.txt").trim().split(',').for_each(|s| {
        if s.find('=').is_some() {
            let (label, num_str) = s.split_once("=").expect("failed to split at =");
            let hash = label
                .bytes()
                .fold(0, |acc, ch| (acc + ch as usize) * 17 % 256);
            if let Some(record) = boxes[hash]
                .iter_mut()
                .find(|record: &&mut (&str, &str)| label == record.0)
            {
                record.1 = num_str;
            } else {
                boxes[hash].push((label, num_str));
            }
        } else {
            let label = s.strip_suffix("-").expect("failed to strip suffix -");
            let hash = label
                .bytes()
                .fold(0, |acc, ch| (acc + ch as usize) * 17 % 256);
            if let Some(idx) = boxes[hash].iter().position(|&(l, _)| label == l) {
                boxes[hash].remove(idx);
            }
        }
    });
    let ans = boxes
        .into_iter()
        .enumerate()
        .map(|(i, bx)| {
            bx.into_iter()
                .enumerate()
                .map(|(j, record)| {
                    (i + 1) * (j + 1) * record.1.parse::<usize>().expect("failed to parse num")
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", ans);
}
