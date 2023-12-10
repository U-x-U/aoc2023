use std::cmp::Ordering;
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
struct Card(char);
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord_arr = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        let r1 = ord_arr
            .iter()
            .position(|&ch| ch == self.0)
            .expect("card val not found");
        let r2 = ord_arr
            .iter()
            .position(|&ch| ch == other.0)
            .expect("card val not found");
        r1.cmp(&r2)
    }
}

fn main() {
    // let input = include_str!("test.txt");
    let input = include_str!("input.txt");
    let cards = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(cards_str, num_str)| {
                    let cards = cards_str.chars().map(|c| Card(c)).collect::<Vec<_>>();
                    (
                        cards,
                        num_str.parse::<u64>().expect("failed to parse number"),
                    )
                })
                .expect("failed to split at space")
        })
        .collect::<Vec<_>>();
    let calc_kind = |cards: &Vec<Card>| -> Vec<(isize, Card)> {
        let mut kinds = vec![];
        assert!(cards.len() == 5);

        let mut cards = cards.clone();
        cards.sort_by(|a, b| b.cmp(&a));
        if cards[0] == cards[4] {
            kinds.push((5, cards[0]));
        } else if cards[0] == cards[3] || cards[1] == cards[4] {
            kinds.push((4, cards[1]));
        } else {
            for i in 0..3 {
                if cards[i] == cards[i + 2] {
                    kinds.push((3, cards[2]));
                }
            }
            for i in 0..4 {
                if (i == 0 || cards[i] != cards[i - 1])
                    && cards[i] == cards[i + 1]
                    && (i == 3 || cards[i] != cards[i + 2])
                {
                    kinds.push((2, cards[i]));
                }
            }
        }
        for i in 0..5 {
            if (i == 0 || cards[i] != cards[i - 1]) && (i == 4 || cards[i] != cards[i + 1]) {
                kinds.push((1, cards[i]));
            }
        }
        kinds
    };
    let mut cards = cards
        .into_iter()
        .map(|(cards, bid)| {
            let kind = calc_kind(&cards);
            let k = kind.iter().map(|(x, _)| *x).collect::<Vec<_>>();
            (k, cards, bid)
        })
        .collect::<Vec<_>>();
    cards.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    // cards.sort();
    println!("{:?}", cards);
    println!(
        "{}",
        cards
            .into_iter()
            .enumerate()
            .fold(0, |acc, (idx, (_, _, bid))| {
                acc + (idx as u64 + 1) * bid
            })
    );
}
