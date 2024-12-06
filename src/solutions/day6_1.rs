use std::collections::{HashMap, HashSet};
use std::error::Error;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_next_position(&self, pos: Position) -> Option<Position> {
        let (r, c) = pos;
        match self {
            Direction::Up if r > 0 => Some((r - 1, c)),
            Direction::Right => Some((r, c + 1)),
            Direction::Down => Some((r + 1, c)),
            Direction::Left if c > 0 => Some((r, c - 1)),
            _ => None,
        }
    }
}

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let grid = parse_grid(input);
    let mut positions = HashSet::new();
    
    // Find guard's starting position and direction
    let mut current_pos = find_guard(&grid)?;
    let mut current_dir = get_initial_direction(&grid, current_pos)?;
    
    positions.insert(current_pos);
    
    loop {
        if let Some(next_pos) = current_dir.get_next_position(current_pos) {
            if is_in_bounds(next_pos, &grid) {
                // Check if there's an obstacle
                if grid[next_pos.0][next_pos.1] == '#' {
                    // Turn right if obstacle ahead
                    current_dir = current_dir.turn_right();
                } else {
                    // Move forward
                    current_pos = next_pos;
                    positions.insert(current_pos);
                }
            } else {
                // Guard has left the area
                break;
            }
        } else {
            break;
        }
    }

    Ok(positions.len() as i64)
}

fn find_guard(grid: &Grid) -> Result<Position, Box<dyn Error>> {
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if matches!(cell, '^' | 'v' | '<' | '>') {
                return Ok((r, c));
            }
        }
    }
    Err("Guard not found".into())
}

fn get_initial_direction(grid: &Grid, pos: Position) -> Result<Direction, Box<dyn Error>> {
    match grid[pos.0][pos.1] {
        '^' => Ok(Direction::Up),
        '>' => Ok(Direction::Right),
        'v' => Ok(Direction::Down),
        '<' => Ok(Direction::Left),
        _ => Err("Invalid direction".into()),
    }
}

fn is_in_bounds(pos: Position, grid: &Grid) -> bool {
    pos.0 < grid.len() && pos.1 < grid[0].len()
}

fn parse_grid(input: &str) -> Grid {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}