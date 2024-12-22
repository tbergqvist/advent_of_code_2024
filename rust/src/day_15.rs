use itertools::Itertools;

fn move_thing(thing: char, map: &mut Vec<Vec<char>>, pos: (usize, usize), direction: (i32, i32)) -> Option<(usize, usize)> {
    let new_pos = ((pos.0 as i32 + direction.0) as usize, (pos.1 as i32 + direction.1) as usize);

    let can_move = match map[new_pos.0][new_pos.1] {
        '#' => false,
        'O' => move_thing('O', map, new_pos, direction).is_some(),
        _ => true,
    };

    if !can_move {
        return None;
    }

    map[new_pos.0][new_pos.1] = thing;
    map[pos.0][pos.1] = '.';
    Some(new_pos)
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{}", row.into_iter().collect::<String>());
    }
}

pub fn a(input: &str) -> usize {
    let (map, commands) = input.split("\n\n").collect_tuple().unwrap();

    let mut map = map.lines().map(|line| line.chars().collect_vec()).collect_vec();

    let mut robot_pos = map.iter().enumerate().find_map(|(y, row)| 
        row.iter().enumerate().find_map(move |(x, c)| 
            if c == &'@' {
                Some((y, x))
            } else {
                None
            })
        ).unwrap();

    for command in commands.lines().flat_map(|l| l.chars()) {
        let direction = match command {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!("Invalid command"),
        };

        if let Some(new_pos) = move_thing('@', &mut map, robot_pos, direction) {
            robot_pos = new_pos;
        }
        /*
        println!("command: {}", command);
        print_map(&map);
         */
    }

    map.iter().enumerate().flat_map(|(y, row)| 
        row.iter().enumerate().filter_map(move |(x, c)| 
            if c == &'O' {
                Some((y, x))
            } else {
                None
            }
        )
    )
    .map(|(y, x)| 100 * y + x)
    .sum()
}

fn can_move(thing: char, map: &Vec<Vec<char>>, pos: (usize, usize), direction: (i32, i32)) -> bool {
    let new_pos = ((pos.0 as i32 + direction.0) as usize, (pos.1 as i32 + direction.1) as usize);

    match (map[new_pos.0][new_pos.1], direction) {
        ('#', _) => false,
        ('[', (_, 1)) => can_move(']', map, (new_pos.0, new_pos.1 + 1), direction),
        ('[', (_, -1)) => can_move(thing, map, new_pos, direction),
        ('[', (1, _) | (-1, _)) => can_move(thing, map, new_pos, direction) && can_move(']', map, (new_pos.0, new_pos.1 + 1), direction),
        (']', (_, 1)) => can_move(thing, map, new_pos, direction),
        (']', (_, -1)) => can_move('[', map, (new_pos.0, new_pos.1 - 1), direction),
        (']', (1, _) | (-1, _)) => can_move(thing, map, new_pos, direction) && can_move('[', map, (new_pos.0, new_pos.1 - 1), direction),
        _ => true,
    }
}

fn move_thing2(thing: char, map: &mut Vec<Vec<char>>, pos: (usize, usize), direction: (i32, i32)) -> Option<(usize, usize)> {
    let new_pos = ((pos.0 as i32 + direction.0) as usize, (pos.1 as i32 + direction.1) as usize);

    let mov = can_move(thing, map, pos, direction);

    if !mov {
        return None;
    }

    match (map[new_pos.0][new_pos.1], direction) {
        ('[', (_, 1)) => {
            move_thing2(']', map, (new_pos.0, new_pos.1 + 1), direction);
            move_thing2('[', map, new_pos, direction);
        },
        ('[', (_, _)) => {
            move_thing2('[', map, new_pos, direction);
            move_thing2(']', map, (new_pos.0, new_pos.1 + 1), direction);
        },
        (']', (_, 1)) => {
            move_thing2(']', map, new_pos, direction);
            move_thing2('[', map, (new_pos.0, new_pos.1 - 1), direction);
        },
        (']', (_, _)) => {
            move_thing2('[', map, (new_pos.0, new_pos.1 - 1), direction);
            move_thing2(']', map, new_pos, direction);
        },
        _ => ()
    };
    map[new_pos.0][new_pos.1] = thing;
    map[pos.0][pos.1] = '.';
    Some(new_pos)
}

pub fn b(input: &str) -> usize {
    let (map, commands) = input.split("\n\n").collect_tuple().unwrap();

    let mut map = map
        .lines()
        .map(|line| 
            line.chars()
                .flat_map(|c| match c {
                    '@' => vec!['@', '.'],
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    _ => panic!("Invalid character"),
                })
                .collect_vec()
        )
        .collect_vec();

    let mut robot_pos = map.iter().enumerate().find_map(|(y, row)| 
        row.iter().enumerate().find_map(move |(x, c)| 
            if c == &'@' {
                Some((y, x))
            } else {
                None
            }
        )
    )
    .unwrap();
    
    for command in commands.lines().flat_map(|l| l.chars()) {
        let direction = match command {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!("Invalid command"),
        };

        if let Some(new_pos) = move_thing2('@', &mut map, robot_pos, direction) {
            robot_pos = new_pos;
        }
        
        //println!("command: {}", command);
        //print_map(&map);
    }

    map.iter().enumerate().flat_map(|(y, row)| 
        row.iter().enumerate().filter_map(move |(x, c)| 
            if c == &'[' {
                Some((y, x))
            } else {
                None
            }
        )
    )
    .map(|(y, x)| 100 * y + x)
    .sum()
}
