use itertools::Itertools;

pub fn check_line(lines: &[i32]) -> bool {
    let direction_up = lines[0] < lines[1];

    lines.windows(2).all(|window| {
        let [x, x2] = window.try_into().unwrap();
        let distance = (x - x2).abs();
        if distance > 3 {
            false
        } else if direction_up {
            x < x2
        } else {
            x > x2
        }
    })
}

pub fn a(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let items = line
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect_vec();

            check_line(&items)
        })
        .count()
}

pub fn b(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let items = line
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect_vec();

            (0..items.len()).any(|i| {
                let mut v = items.clone();
                v.remove(i);
                check_line(&v)
            })
        })
        .count()
}
