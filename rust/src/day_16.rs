use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::{astar, astar_bag};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Direction {
	Up,
	Right,
	Down,
	Left
}

pub fn a(input: &str) -> usize {
    let map = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let start_pos = map.iter().enumerate().find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y))).unwrap();
    let end_pos = map.iter().enumerate().find_map(|(y, row)| row.iter().position(|&c| c == 'E').map(|x| (x, y))).unwrap();
    
    astar(&(start_pos, Direction::Right), |&((x, y), last_direction)| {
        let mut next_moves: Vec<((usize, usize), Direction)> = vec![];

        if map[y][x + 1] != '#' && last_direction != Direction::Left {
            next_moves.push(((x + 1, y), Direction::Right));
        }
        if map[y - 1][x] != '#' && last_direction != Direction::Down {
            next_moves.push(((x, y - 1), Direction::Up));
        }
        if map[y][x - 1] != '#' && last_direction != Direction::Right {
            next_moves.push(((x - 1, y), Direction::Left));
        }
        if map[y + 1][x] != '#' && last_direction != Direction::Up {
            next_moves.push(((x, y + 1), Direction::Down));
        }

        next_moves.into_iter().map(move |moves| {
            (moves, if last_direction == moves.1 { 1 } else { 1001 })
        })
    }, 
    |&((x, y), _)| end_pos.0.abs_diff(x) + end_pos.1.abs_diff(y), 
    |&(current_pos, _)| current_pos == end_pos)
    .unwrap()
    .1
}

pub fn b(input: &str) -> usize {
    let map = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let start_pos = map.iter().enumerate().find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y))).unwrap();
    let end_pos = map.iter().enumerate().find_map(|(y, row)| row.iter().position(|&c| c == 'E').map(|x| (x, y))).unwrap();
    
    let all_paths = astar_bag(&(start_pos, Direction::Right), |&((x, y), last_direction)| {
        let mut next_moves: Vec<((usize, usize), Direction)> = vec![];

        if map[y][x + 1] != '#' && last_direction != Direction::Left {
            next_moves.push(((x + 1, y), Direction::Right));
        }
        if map[y - 1][x] != '#' && last_direction != Direction::Down {
            next_moves.push(((x, y - 1), Direction::Up));
        }
        if map[y][x - 1] != '#' && last_direction != Direction::Right {
            next_moves.push(((x - 1, y), Direction::Left));
        }
        if map[y + 1][x] != '#' && last_direction != Direction::Up {
            next_moves.push(((x, y + 1), Direction::Down));
        }

        next_moves.into_iter().map(move |moves| {
            (moves, if last_direction == moves.1 { 1 } else { 1001 })
        })
    }, 
    |&((x, y), _)| end_pos.0.abs_diff(x) + end_pos.1.abs_diff(y), 
    |&(current_pos, _)| current_pos == end_pos)
    .unwrap()
    .0
    .flat_map(|a| a.into_iter().map(|b| b.0))
    .fold(HashMap::new(), |mut acc, pos| {
        *acc.entry(pos).or_insert(1) += 1;
        acc
    });

    all_paths.values().into_iter()
        .filter(|count| count > &&1)
        .count()
}