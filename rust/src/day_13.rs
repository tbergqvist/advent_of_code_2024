use itertools::Itertools;

struct Machine {
    x_mov_a: i64,
    y_mov_a: i64,
    x_mov_b: i64,
    y_mov_b: i64,
    win_x: i64,
    win_y: i64,
}

fn parse_machines(input: &str, offset: i64) -> Vec<Machine> {
    let num_finder = regex::Regex::new(r"(\d+)").unwrap();

    input
        .split("\n\n")
        .map(|machine_lines| {
            let (a_line, b_line, price_line) = machine_lines.lines()
            .collect_tuple()
            .unwrap();

            let (x_a, y_a) = num_finder.find_iter(a_line).map(|i| i.as_str().parse().unwrap()).collect_tuple().unwrap();
            let (x_b, y_b) = num_finder.find_iter(b_line).map(|i| i.as_str().parse().unwrap()).collect_tuple().unwrap();
            let (x_price, y_price) = num_finder.find_iter(price_line).map(|i| i.as_str().parse::<i64>().unwrap()).collect_tuple().unwrap();
            Machine {
                x_mov_a: x_a,
                y_mov_a: y_a,
                x_mov_b: x_b,
                y_mov_b: y_b,
                win_x: x_price + offset,
                win_y: y_price + offset,
            }
        }
        )
        .collect()
}

pub fn solve(input: &str, offset: i64) -> i64 {
    let machines = parse_machines(input, offset);

    machines.into_iter().filter_map(|machine| {
        let x_div = machine.win_x * machine.y_mov_b - machine.win_y * machine.x_mov_b;
        let x_val = machine.x_mov_a * machine.y_mov_b - machine.y_mov_a * machine.x_mov_b;
    
        if x_div % x_val != 0 {
            return None;
        }
    
        let a_presses = x_div / x_val;
        let y_div = machine.win_x - a_presses * machine.x_mov_a;
        let y_val = machine.x_mov_b;

        if y_div % y_val != 0 {
            return None;
        }

        let b_presses = y_div / y_val;
    
        Some(a_presses * 3 + b_presses)
         
    })
    .sum()
}

pub fn a(input: &str) -> i64 {
    solve(input, 0)
}
pub fn b(input: &str) -> i64 {
    solve(input, 10000000000000)
}