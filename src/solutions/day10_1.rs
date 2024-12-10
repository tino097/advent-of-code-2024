use std::error::Error;
use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<u32>>;
type Position = (usize, usize);

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let grid = parse_grid(input);
    let trailheads = find_trailheads(&grid);
    let mut total_score = 0;
    for trailhead in trailheads {
        let score = calculate_trailhead_score(&grid, trailhead);
        total_score += score;
    }

    Ok(total_score)
}

fn parse_grid(input: &str) -> Grid {
    input.lines()
        .map(|line| 
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        )
        .collect()
}

fn find_trailheads(grid: &Grid) -> Vec<Position> {
    let mut trailheads = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, &height) in line.iter().enumerate() {
            if height == 0 {
                trailheads.push((row, col));
            }
        }
    }
    trailheads
}

fn calculate_trailhead_score(grid: &Grid, trailhead: Position) -> i64 {
	let mut reachable_nines = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(trailhead);

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        let current_height = grid[current.0][current.1];
        
        // If we reached a 9, count it
        if current_height == 9 {
            reachable_nines.insert(current);
            continue;
        }

        for neighbor in neighbors(current, grid) {
            let neighbor_height = grid[neighbor.0][neighbor.1];
            if neighbor_height == current_height + 1 {
                queue.push_back(neighbor);
            }
        }
    }

    reachable_nines.len() as i64
}

fn neighbors((row, col): Position, grid: &Grid) -> Vec<Position> {
	let mut neighbors = Vec::new();
	if row > 0 {
		neighbors.push((row - 1, col));
	}
	if col > 0 {
		neighbors.push((row, col - 1));
	}
	if row < grid.len() - 1 {
		neighbors.push((row + 1, col));
	}
	if col < grid[0].len() - 1 {
		neighbors.push((row, col + 1));
	}
	neighbors
}