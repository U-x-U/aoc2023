fn main() {
    let input = include_str!("input.txt");
    let ans = input.lines().fold(0, |acc, line| {
        acc + {
            let (_, games) = line.split_once(':').expect("failed to split at colon");
            let games = games.split(';').collect::<Vec<_>>();
            let (r, g, b) = games
                .iter()
                .map(|game| {
                    let (mut r, mut g, mut b) = (0, 0, 0);
                    for cnt_color in game.split(',') {
                        let cnt_color = cnt_color.trim();
                        let (cnt, color) = cnt_color
                            .split_once(' ')
                            .expect("failed to parse count and color");
                        let cnt = cnt.parse::<usize>().expect("failed to parse count");
                        match color {
                            "red" => r += cnt,
                            "green" => g += cnt,
                            "blue" => b += cnt,
                            _ => unreachable!(),
                        }
                    }
                    (r, g, b)
                })
                .fold((0, 0, 0), |(ar, ag, ab), (r, g, b)| {
                    (ar.max(r), ag.max(g), ab.max(b))
                });
            r * g * b
        }
    });
    println!("{}", ans);
}
