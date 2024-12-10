use std::error::Error;
use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<u32>>;
type Position = (usize, usize);

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
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

	fn pathfinder(grid: &Grid, pos: Position, visited: &mut HashSet<Position>) -> i64 {
        // If we've reached height 9, we've found a valid path
        if grid[pos.0][pos.1] == 9 {
            return 1;
        }
        
        let current_height = grid[pos.0][pos.1];
        let mut paths = 0;
        
        // Try all neighbors
        for next in neighbors(pos, grid) {
            let next_height = grid[next.0][next.1];
            
            // Only follow paths that increase by exactly 1
            if next_height == current_height + 1 && !visited.contains(&next) {
                visited.insert(next);
                paths += pathfinder(grid, next, visited);
                visited.remove(&next);
            }
        }
        
        paths
    }

    let mut visited = HashSet::new();
    visited.insert(trailhead);
    pathfinder(grid, trailhead, &mut visited)
	
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