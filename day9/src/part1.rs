use aoc_core::read;

fn main() {
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

    let empty_iter: Vec<usize> = disk
        .iter()
        .enumerate()
        .filter(|(_, d)| d.is_none())
        .map(|(i, _)| i)
        .collect();

    let reverse_file_iter: Vec<usize> = disk
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, d)| d.is_some())
        .map(|(i, _)| i)
        .collect();

    let mut is_cleaned = false;
    let mut clean_i = 0;

    while !is_cleaned {
        let empty_i = empty_iter[clean_i];
        let file_i = reverse_file_iter[clean_i];

        is_cleaned = empty_i >= file_i;

        if !is_cleaned {
            disk.swap(empty_i, file_i);
        }

        clean_i += 1;
    }

    let checksum = get_checksum(&disk);

    println!("{:?}", checksum);
}

fn get_checksum(disk: &Vec<Option<usize>>) -> usize {
    disk.iter()
        .enumerate()
        .filter(|(_, d)| d.is_some())
        .map(|(i, d)| i * d.unwrap())
        .sum()
}
