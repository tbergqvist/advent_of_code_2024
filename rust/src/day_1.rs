use std::collections::HashMap;

use itertools::Itertools;

fn input_to_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| 
            line
                .split_whitespace()
                .map(|char| char.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        )
        .unzip()
}

pub fn a(input: &str) -> i32 {
    let (mut first_list, mut second_list) = input_to_lists(input);

    first_list.sort_unstable();
    second_list.sort_unstable();
    
    first_list.into_iter()
        .zip(second_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn b(input: &str) -> i32 {
    let (first_list, second_list) = input_to_lists(input);
    
    let count_map = second_list.iter()
        .fold(HashMap::new(), |mut count_map, value| {
            count_map.entry(value).and_modify(|i| *i += 1).or_insert(1);
            count_map
        });

    first_list.into_iter()
        .map(|value| value * count_map.get(&value).unwrap_or(&0))
        .sum()
}