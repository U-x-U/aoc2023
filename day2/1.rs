fn main() {
    let input = include_str!("input.txt");
    let ans = input.lines().fold(0, |acc, line| {
        acc + {
            let (id_str, games) = line.split_once(':').expect("failed to split at colon");
            let id = id_str
                .trim_start_matches("Game ")
                .trim()
                .parse::<usize>()
                .expect("failed to parse id");
            let games = games.split(';').collect::<Vec<_>>();
            games
                .iter()
                .all(|game| {
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
                    r <= 12 && g <= 13 && b <= 14
                })
                .then(|| id)
                .unwrap_or(0)
        }
    });
    println!("{}", ans);
}
