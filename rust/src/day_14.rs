use std::{thread, time::Duration};

use itertools::Itertools;

#[derive(Debug)]
struct Robot {
    position: (i16, i16),
    velocity: (i16, i16),
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let num_finder = regex::Regex::new(r"-?\d+").unwrap();

    input
        .lines()
        .map(|line| {
            let (px, py, vx, vy) = num_finder.find_iter(line).map(|i| i.as_str().parse().unwrap()).collect_tuple().unwrap();
            Robot {
                position: (px, py),
                velocity: (vx, vy),
            }
        })
        .collect()
}

pub fn a(input: &str) -> i64 {
    let width = 101;
    let height = 103;
    let robots = parse_robots(input);

    robots.into_iter().map(|mut robot| {
        for _ in 0..100 {
            robot.position.0 = (robot.position.0 + robot.velocity.0) % width;
            robot.position.1 = (robot.position.1 + robot.velocity.1) % height;

            if robot.position.0 < 0 {
                robot.position.0 += width;
            }

            if robot.position.1 < 0 {
                robot.position.1 += height;
            }
        }
        robot.position
    }).fold(vec![0, 0, 0, 0], |mut quadrants, position| {
        if position.0 < width / 2 && position.1 < height / 2 {
            quadrants[0] += 1;
        } else if position.0 > width / 2 && position.1 < height / 2 {
            quadrants[1] += 1;
        } else if position.0 < width / 2 && position.1 > height / 2 {
            quadrants[2] += 1;
        } else if position.0 > width / 2 && position.1 > height / 2 {
            quadrants[3] += 1;
        }

        quadrants
    })
    .into_iter()
    .reduce(|v, v2| v * v2)
    .unwrap()
}

pub fn b(input: &str) -> i64 {
    /*    
    let width = 101;
    let height = 103;
    let mut robots = parse_robots(input);

    for i in 0.. {
        println!("{}[2J", 27 as char);
        println!("{}s", i);
        for robot in &mut robots {
            robot.position.0 = (robot.position.0 + robot.velocity.0) % width;
            robot.position.1 = (robot.position.1 + robot.velocity.1) % height;

            if robot.position.0 < 0 {
                robot.position.0 += width;
            }

            if robot.position.1 < 0 {
                robot.position.1 += height;
            }
        }

        let mut grid = vec![vec!['.'; width as usize]; height as usize];
        for robot in &robots {
            grid[robot.position.1 as usize][robot.position.0 as usize] = '#';
        }

        for row in grid {
            println!("{}", row.into_iter().collect::<String>());
        }
        
        if i > 8000 {
            thread::sleep(Duration::from_millis(1000));
        }
    }
     */

    let mut height_start = 27;
    let height_diff= 101;
    let mut width_start = 75;
    let width_diff = 103;

    for _ in 0.. {
        while height_start < width_start {
            height_start += height_diff;
        }
        while width_start < height_start {
            width_start += width_diff;
        }
        if height_start == width_start {
            return height_start;
        }
    };

    0
}
