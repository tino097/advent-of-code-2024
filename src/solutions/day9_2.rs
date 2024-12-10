use std::error::Error;

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let lines: Vec<&str> = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    let line: Vec<usize> = lines[0]
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect();

    let mut disk = Vec::new();
    let mut space_map = Vec::new();
    let mut idx = 0;

    // Build the disk layout and space map
    for chunk in line.chunks(2) {
        let block_size = chunk[0];
        disk.extend(vec![Some(idx); block_size]);

        if let Some(&space_size) = chunk.get(1) {
            if space_size > 0 {
                space_map.push((disk.len(), disk.len() + space_size));
                disk.extend(vec![None; space_size]);
            }
        }

        idx += 1;
    }

    let mut block_idx_start = disk.len().saturating_sub(1);

    while block_idx_start >= 0 {
        let mut cur_block = None;
        let mut block_idx_end = 0;

        // Find the next block to move
        while block_idx_start < disk.len() && block_idx_start > 0 {
            if cur_block.is_none() && disk[block_idx_start].is_some() {
                cur_block = disk[block_idx_start];
                block_idx_end = block_idx_start + 1;
            } else if cur_block.is_some() && disk[block_idx_start] != cur_block {
                block_idx_start += 1;
                break;
            }

            if block_idx_start == 0 {
                break;
            }
        }

        let block_len = block_idx_end - block_idx_start;
        if block_len == 0 {
            break;
        }

        // Try to move the block into free space
        for i in 0..space_map.len() {
            let (s, e) = space_map[i];
            if e - s >= block_len && block_idx_start > s {
                // Split disk at s, which allows us to get a mutable borrow of the left part
                let (left, right) = disk.split_at_mut(s);
                
                // Now split the right part into target and remaining slices
                let (target, remaining) = right.split_at_mut(block_len);
                
                // Now we can copy the block into the target part
                target.copy_from_slice(disk[block_idx_start..block_idx_end]);
                
                // We clear the original block's position in the left part
                remaining.fill(None);
                
                // Update the space map
                space_map[i] = (s + block_len, e);
                break;
            }
        }

        if block_idx_start == 0 {
            break;
        }
        block_idx_start = block_idx_start.saturating_sub(1);
    }

    // Calculate checksum
    let sum: i64 = disk
        .iter()
        .enumerate()
        .filter_map(|(i, &block)| block.map(|b| i as i64 * b as i64))
        .sum();

    Ok(sum)
}
