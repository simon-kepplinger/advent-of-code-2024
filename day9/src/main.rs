use aoc_core::{end_measure, read, start_measure};

fn main() {
    let mes = start_measure();

    let input = read("in/input");

    let mut disk: Vec<Option<usize>> = Vec::new();

    for (i, c) in input.chars().enumerate() {
        let amount = c.to_digit(10).unwrap();

        if i % 2 == 0 {
            let file: Vec<_> = (0..amount).map(|_| Some(i / 2)).collect();

            disk.extend(file)
        } else {
            disk.extend((0..amount).map(|_| None))
        }
    }

    let mut file_i = disk.len();

    while file_i > 0 {
        let file_chunk = get_next_file_chunk(&disk, file_i);
        file_i = file_chunk[0];

        let mut empty_i = 0;

        while empty_i < file_i {
            let empty_chunk = get_next_empty_chunk(&disk, empty_i + 1, file_i);

            if empty_chunk.is_empty() {
                break;
            }

            if file_chunk.len() <= empty_chunk.len() {
                for swap_i in 0..file_chunk.len() {
                    disk.swap(file_chunk[swap_i], empty_chunk[swap_i]);
                }

                break;
            }

            empty_i = *empty_chunk.last().unwrap();
        }
    }

    let checksum = get_checksum(&disk);

    println!("{:?}", checksum);

    end_measure(mes)
}

fn get_next_file_chunk(disk: &[Option<usize>], from: usize) -> Vec<usize> {
    let mut chunk = vec![];
    let mut matching_file: usize = 0;

    for i in (0..from).rev() {
        if let Some(v) = disk[i] {
            if chunk.is_empty() {
                matching_file = v;
                chunk.insert(0, i);
            } else if v == matching_file {
                chunk.insert(0, i);
            } else {
                return chunk;
            }
        }
    }

    chunk
}

fn get_next_empty_chunk(disk: &[Option<usize>], from: usize, to: usize) -> Vec<usize> {
    let mut chunk = vec![];

    for i in from..to {
        match disk[i] {
            None => chunk.push(i),
            Some(_) => {
                if !chunk.is_empty() {
                    return chunk;
                }
            }
        }
    }

    chunk
}

fn get_checksum(disk: &Vec<Option<usize>>) -> usize {
    disk.iter()
        .enumerate()
        .filter(|(_, d)| d.is_some())
        .map(|(i, d)| i * d.unwrap())
        .sum()
}
