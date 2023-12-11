fn main() {
    let mut image = include_str!("input.txt")
        // let mut image = include_str!("test.txt")
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    {
        let mut i = 0;
        while i < image.len() {
            if image[i].iter().all(|&cell| cell == '.') {
                image.insert(i, vec!['.'; image[i].len()]);
                i += 1;
            }
            i += 1;
        }
        i = 0;
        while i < image[0].len() {
            let mut is_empty = true;
            for j in 0..image.len() {
                if image[j][i] != '.' {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                for j in 0..image.len() {
                    image[j].insert(i, '.');
                }
                i += 1;
            }
            i += 1;
        }
    }
    let mut galaxies = vec![];
    for i in 0..image.len() {
        for j in 0..image[i].len() {
            if image[i][j] == '#' {
                galaxies.push((i as isize, j as isize));
            }
        }
    }
    let mut ans = 0;
    let calc_dist = |(x1, y1): (isize, isize), (x2, y2): (isize, isize)| -> isize {
        (x1 - x2).abs() + (y1 - y2).abs()
    };
    for i in 0..galaxies.len() {
        for j in 0..i {
            ans += calc_dist(galaxies[i], galaxies[j]);
        }
    }
    println!("{}", ans);
}
