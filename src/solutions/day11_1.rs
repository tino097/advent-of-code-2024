use std::error::Error;

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut stones: Vec<String> = input.split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    for _ in 0..25 {
        stones = transform(stones);
    }
    
    Ok(stones.len() as i64)
} 

fn transform(stones: Vec<String>) -> Vec<String> {
    let mut new_stones = Vec::new();
    
    for stone in stones {
        if stone == "0" {
            new_stones.push("1".to_string());
        } else if stone.len() % 2 == 0 {
            // Rule 2: Split even-length numbers in half
            let half = stone.len() / 2;
            let left = stone[..half].trim_start_matches('0').to_string();
            let right = stone[half..].trim_start_matches('0').to_string();
            
            if left.is_empty() {
                new_stones.push("0".to_string());
            } else {
                new_stones.push(left);
            }
            if right.is_empty() {
                new_stones.push("0".to_string());
            } else {
                new_stones.push(right);
            }
        } else {
            // Rule 3: Multiply by 2024
            if let Ok(num) = stone.parse::<i64>() {
                new_stones.push((num * 2024).to_string());
            }
        }
    }

    new_stones
}