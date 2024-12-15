use std::error::Error;
use std::collections::HashMap;

type MemoKey = (i64, usize);  // (stone_value, blinks_remaining)
type MemoMap = HashMap<MemoKey, usize>;

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let stones: Vec<i64> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let mut memo = HashMap::new();
    let blinks = 75;

    let total: usize = stones.iter()
        .enumerate()
        .map(|(i, &stone)| {
            let count = count_stones_recursive(stone, blinks, &mut memo);
            println!("Stone {} processed: {} patterns", i + 1, count);
            count
        })
        .sum();

    Ok(total as i64)
}

fn count_stones_recursive(stone: i64, blinks: usize, memo: &mut MemoMap) -> usize {
    if blinks == 0 {
        return 1;
    }

    let key = (stone, blinks);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    // Calculate based on rules
    let result = if stone == 0 {
        count_stones_recursive(1, blinks - 1, memo)
    } else {
        let digits = get_digit_count(stone);
        if digits % 2 == 0 {
            let (left, right) = split_number(stone, digits);
            count_stones_recursive(left, blinks - 1, memo) +
            count_stones_recursive(right, blinks - 1, memo)
        } else {
            count_stones_recursive(stone * 2024, blinks - 1, memo)
        }
    };

    memo.insert(key, result);
    result
}

// Helper functions to avoid string conversions
fn get_digit_count(mut n: i64) -> usize {
    let mut count = 0;
    if n == 0 { return 1; }
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

fn split_number(n: i64, digits: usize) -> (i64, i64) {
    let half_digits = digits / 2;
    let divisor = 10i64.pow(half_digits as u32);
    let right = n % divisor;
    let left = n / divisor;
    (left, right)
}