use std::collections::{HashMap, HashSet};
use std::error::Error;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
   let mut grid = parse_grid(input);
   let start_pos = find_guard(&grid)?;
   
   let mut loop_positions = HashSet::new();
   
   for r in 0..grid.len() {
       for c in 0..grid[0].len() {
           if grid[r][c] == '.' && (r,c) != start_pos {
               // Place obstacle
               grid[r][c] = '#';
               
               // Check if this creates a loop
               if creates_loop(&grid, start_pos) {
                   loop_positions.insert((r,c));
               }
               // Remove obsticle
               grid[r][c] = '.';
           }
       }
   }

   Ok(loop_positions.len() as i64)
}

fn creates_loop(grid: &Grid, start_pos: Position) -> bool {
   let mut current_pos = start_pos;
   let mut current_dir = get_initial_direction(grid, start_pos).unwrap();
   let mut visited = HashSet::new();
   visited.insert((current_pos, current_dir));
   
   loop {
       if let Some(next_pos) = current_dir.get_next_position(current_pos) {
           if is_in_bounds(next_pos, grid) {
               if grid[next_pos.0][next_pos.1] == '#' {
                   current_dir = current_dir.turn_right();
               } else {
                   current_pos = next_pos;
               }
               
               // Check if this position and direction been visited before
               if visited.contains(&(current_pos, current_dir)) {
                   return true;
               }
               visited.insert((current_pos, current_dir));
           } else {
               return false;
           }
       } else {
           return false;
       }
   }
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