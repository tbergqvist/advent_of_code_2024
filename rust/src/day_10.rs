use itertools::Itertools;

fn find_start_positions(map: &Vec<Vec<u8>>) -> Vec<(i32, i32)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| 
            row.iter()
                .enumerate()
                .filter_map(|(x, b)| (*b == b'0').then_some(x))
                .map(move |x| (y as i32, x as i32))
        )
        .collect_vec()
}

fn parse_map(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().collect_vec()
        )
        .collect_vec()
}

pub fn walk(map: &Vec<Vec<u8>>, y: i32, x: i32, current_value: u8) -> Vec<(i32, i32)> {
    if current_value == b'9' {
        return vec![(y, x)];
    }

    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .map(|(dy, dx)| (y + dy, x + dx))
        .filter(|(ny, nx)| *ny >= 0 && *nx >= 0 && *ny < map.len() as i32 && *nx < map[0].len() as i32)
        .filter(|(ny, nx)| map[*ny as usize][*nx as usize] == (current_value + 1))
        .flat_map(|(ny, nx)| walk(map, ny, nx, current_value + 1))
        .collect_vec()
}

pub fn a(input: &str) -> usize {
    let map = parse_map(input);
    let start_positions = find_start_positions(&map);

    start_positions.into_iter()
        .map(|(y, x)| walk(&map, y, x, b'0')
            .into_iter()
            .unique()
            .count())
        .sum()
}

pub fn b(input: &str) -> usize {
    let map = parse_map(input);
    let start_positions = find_start_positions(&map);

    start_positions.into_iter()
        .map(|(y, x)| walk(&map, y, x, b'0')
            .into_iter()
            .count()
        )
        .sum()
}
