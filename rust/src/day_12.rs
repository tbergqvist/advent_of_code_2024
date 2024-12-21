use std::collections::HashSet;

use itertools::Itertools;

struct Field {
    borders: usize,
    area: usize,
}

type Point = (usize, usize);

fn visit(map: &Vec<Vec<u8>>, y: usize, x: usize, visited: &mut HashSet<Point>) -> Field {
    if visited.contains(&(y, x)) {
        return Field {
            borders: 0,
            area: 0,
        };
    }
    visited.insert((y, x));

    let current_value = map[y as usize][x as usize];

    let y = y as i32;
    let x = x as i32;

    let neighbors = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .map(|(dy, dx)| (y + dy, x + dx))
        .filter(|(ny, nx)| *ny >= 0 && *nx >= 0 && *ny < map.len() as i32 && *nx < map[0].len() as i32)
        .filter(|(ny, nx)| map[*ny as usize][*nx as usize] == current_value)
        .collect_vec();

    neighbors
        .iter()
        .map(|&(ny, nx)| visit(map, ny as usize, nx as usize, visited))
        .fold(Field {
            borders: 4 - neighbors.len(),
            area: 1,
        }, |acc, field| Field {
            borders: acc.borders + field.borders,
            area: acc.area + field.area,
        })
}

pub fn a(input: &str) -> usize {
    let map = input.lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();

    let mut visited: HashSet<Point> = HashSet::new();

    (0..map.len()).into_iter().flat_map(|y| {
        (0..map[y].len()).into_iter().map(move |x| {
            (y, x)
        })
    })
    .filter_map(|(y, x)| {
        if visited.contains(&(y, x)) {
            return None;
        }
        Some(visit(&map, y, x, &mut visited))
    })
    .map(|field| field.borders * field.area)
    .sum()
}

fn visit2(map: &Vec<Vec<u8>>, y: usize, x: usize, visited: &mut HashSet<Point>) -> Field {
    if visited.contains(&(y, x)) {
        return Field {
            borders: 0,
            area: 0,
        };
    }
    visited.insert((y, x));

    let current_value = map[y as usize][x as usize];

    let y = y as i32;
    let x = x as i32;

    let neighbors = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .map(|(dy, dx)| ((dy, dx), (y + dy, x + dx)))
        .filter(|(_, (ny, nx))| *ny >= 0 && *nx >= 0 && *ny < map.len() as i32 && *nx < map[0].len() as i32)
        .filter(|(_, (ny, nx))| map[*ny as usize][*nx as usize] == current_value)
        .collect_vec();

    let corners = [((0, -1), (-1, 0)), ((0, 1), (-1, 0)), ((0, 1), (1, 0)), ((0, -1), (1, 0))]
        .into_iter()
        .filter(|(dir1, dir2)| {
            let diagonal_pos = (dir1.0 + dir2.0 + y, dir1.1 + dir2.1 + x);
            
            let diagonal = diagonal_pos.0 >= 0 && diagonal_pos.1 >= 0 && diagonal_pos.0 < map.len() as i32 && diagonal_pos.1 < map[0].len() as i32 && map[diagonal_pos.0 as usize][diagonal_pos.1 as usize] == current_value;
            
            let has_dir1 = neighbors.iter().any(|(direction, _)| direction == dir1);
            let has_dir2 = neighbors.iter().any(|(direction, _)| direction == dir2);

            (!has_dir1 && !has_dir2) || (!diagonal && has_dir1 && has_dir2)
        }
        )
        .collect_vec();

    neighbors
        .iter()
        .map(|&(_, (ny, nx))| visit2(map, ny as usize, nx as usize, visited))
        .fold(Field {
            borders: corners.len(),
            area: 1,
        }, |acc, field| Field {
            borders: acc.borders + field.borders,
            area: acc.area + field.area,
        })
}

pub fn b(input: &str) -> usize {
    let map = input.lines()
    .map(|line| line.bytes().collect_vec())
    .collect_vec();

    let mut visited: HashSet<Point> = HashSet::new();

    (0..map.len()).into_iter().flat_map(|y| {
        (0..map[y].len()).into_iter().map(move |x| {
            (y, x)
        })
    })
    .filter_map(|(y, x)| {
        if visited.contains(&(y, x)) {
            return None;
        }
        Some(visit2(&map, y, x, &mut visited))
    })
    .map(|field| field.borders * field.area)
    .sum()
}
