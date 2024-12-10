use itertools::Itertools;

pub fn a(input: &str) -> u64 {
    let mut blocks = input
        .chars()
        .filter_map(|line| line.to_digit(10))
        .collect_vec()
        .chunks(2)
        .enumerate()
        .flat_map(|(i, chunk)| {
            let file_size = *chunk.get(0).unwrap();
            let empty_size = *chunk.get(1).unwrap_or(&0);

            (0..file_size).into_iter().map(move |_| Some(i))
                .chain((0..empty_size).into_iter().map(|_| None))

        })
        .collect_vec();

    let mut file_index = blocks.len() - 1;
    let mut empty_index = 0;

    loop {
        while blocks[file_index].is_none() {
            file_index -= 1;
        }
        
        while blocks[empty_index].is_some() {
            empty_index += 1;
        }

        if file_index <= empty_index {
            break;
        }

        blocks.swap(file_index, empty_index);
    }

    blocks.into_iter()
        .filter_map(|b| b)
        .enumerate()
        .map(|(i, id)| (i * id) as u64)
        .sum()
}

pub fn b(input: &str) -> u64 {
    let mut blocks = input
    .chars()
    .filter_map(|line| line.to_digit(10))
    .collect_vec()
    .chunks(2)
    .enumerate()
    .flat_map(|(i, chunk)| {
        let file_size = *chunk.get(0).unwrap();
        let empty_size = *chunk.get(1).unwrap_or(&0);

        (0..file_size).into_iter().map(move |_| Some(i))
            .chain((0..empty_size).into_iter().map(|_| None))

    })
    .collect_vec();

    let mut file_index = blocks.len() - 1;

    while file_index > 0 {
        while blocks[file_index].is_none() && file_index > 0 {
            file_index -= 1;
        }

        let mut file_start = file_index;
        while blocks[file_start].is_some() && file_start > 0 && blocks[file_start].unwrap() == blocks[file_index].unwrap() {
            file_start -= 1;
        }

        let mut empty_index = 0;
        while empty_index < (blocks.len() - 1) {
            while empty_index < blocks.len() && blocks[empty_index].is_some() {
                empty_index += 1;
            }

            let mut empty_end = empty_index;
            while empty_end < blocks.len() && blocks[empty_end].is_none() {
                empty_end += 1;
            }

            let file_size = file_index - file_start;
            let empty_size = empty_end - empty_index;
            
            if file_start > empty_index && file_size <= empty_size {
                for i in 0..file_size
                {
                    blocks.swap(file_index - i, empty_index + i);
                }

                break;
            }
            empty_index = empty_end;
        }
        file_index = file_start;
    }

    blocks.into_iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|a| (i, a)))
        .map(|(i, id)| (i * id) as u64)
        .sum()
}
