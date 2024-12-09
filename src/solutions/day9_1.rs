use std::error::Error;

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let lines: Vec<&str> = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    let line: Vec<usize> = lines[0]
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize) // Assuming valid digits
        .collect();

    let mut disk = Vec::new();
    let mut idx = 0;

    for chunk in line.chunks(2) {
        let block_size = chunk[0];
        disk.extend(vec![Some(idx); block_size]);

        if let Some(&space_size) = chunk.get(1) {
            disk.extend(vec![None; space_size]);
        }

        idx += 1;
    }

    let mut free_idx = 0;
    let mut block_idx = disk.len().saturating_sub(1);

    while free_idx < block_idx {
        while free_idx < disk.len() && disk[free_idx].is_some() {
            free_idx += 1;
        }
        while block_idx > 0 && disk[block_idx].is_none() {
            block_idx -= 1;
        }
        if free_idx >= block_idx {
            break;
        }
        disk.swap(free_idx, block_idx);
    }

    let sum: i64 = disk
        .iter()
        .enumerate()
        .take_while(|(_, block)| block.is_some())
        .map(|(i, block)| i as i64 * block.unwrap() as i64)
        .sum();

    Ok(sum)
}
