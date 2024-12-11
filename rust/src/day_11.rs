use std::collections::HashMap;

fn blink(input: &str, iterations: u64) -> u64 {
    let mut bucket: HashMap<u64, u64>  = HashMap::new();
    
    input
    .split(" ")
    .filter_map(|s| s.parse().ok())
    .for_each(|v| {
        let count = bucket.entry(v).or_insert(0);
        *count += 1;
    });
    
    for _ in 0..iterations {
        let mut new_bucket = HashMap::new();
        for (num, count) in bucket.iter() {
            match num {
                0 => {
                    let entry = new_bucket.entry(1).or_insert(0);
                    *entry += count;
                },
                1 => {
                    let entry = new_bucket.entry(2024).or_insert(0);
                    *entry += count;
                },
                _ if { num.to_string().len() % 2 == 0 } => {
                    let as_string = num.to_string();
                    let (first, second) = as_string.split_at(as_string.len() / 2);
                    let entry = new_bucket.entry(first.parse().unwrap()).or_insert(0);
                    *entry += count;

                    let entry2 = new_bucket.entry(second.parse().unwrap()).or_insert(0);
                    *entry2 += count;
                }
                _ => {
                    let entry = new_bucket.entry(num * 2024).or_insert(0);
                    *entry += count;
                },
            }
        }

        bucket = new_bucket;
    }

    bucket.values().into_iter().sum()
}

pub fn a(input: &str) -> u64 {
    blink(input, 25)
}

pub fn b(input: &str) -> u64 {
    blink(input, 75)
}
