use std::error::Error;
use std::collections::{HashSet, HashMap};

type Position = (usize, usize);
type Grid = Vec<Vec<char>>;

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
   let grid = parse_grid(input);
   let mut total_price = 0;
   let mut visited = HashSet::new();

   for row in 0..grid.len() {
       for col in 0..grid[0].len() {
           let pos = (row, col);
           if !visited.contains(&pos) {
               // Find the region and calculate its price
               let region = find_region(&grid, pos, &mut visited);
               let area = region.len();
               let perimeter = calculate_perimeter(&grid, &region);
               let price = area as i64 * perimeter as i64;
               
            //    println!("Found region at ({},{}) with plant type {}, area {}, perimeter {}, price {}", 
            //        row, col, grid[row][col], area, perimeter, price);
               
               total_price += price;
           }
       }
   }

   Ok(total_price)
}

fn parse_grid(input: &str) -> Grid {
   input.lines()
       .map(|line| line.chars().collect())
       .collect()
}

fn find_region(grid: &Grid, start: Position, visited: &mut HashSet<Position>) -> HashSet<Position> {
   let mut region = HashSet::new();
   let mut queue = vec![start];
   let plant_type = grid[start.0][start.1];

   while let Some(pos) = queue.pop() {
       if !visited.contains(&pos) && grid[pos.0][pos.1] == plant_type {
           visited.insert(pos);
           region.insert(pos);

           // Add neighbors
           for neighbor in get_neighbors(pos, grid.len(), grid[0].len()) {
               queue.push(neighbor);
           }
       }
   }

   region
}

fn get_neighbors(pos: Position, rows: usize, cols: usize) -> Vec<Position> {
   let (row, col) = pos;
   let mut neighbors = Vec::new();

   if row > 0 { neighbors.push((row-1, col)); }
   if col > 0 { neighbors.push((row, col-1)); }
   if row + 1 < rows { neighbors.push((row+1, col)); }
   if col + 1 < cols { neighbors.push((row, col+1)); }

   neighbors
}

fn calculate_perimeter(grid: &Grid, region: &HashSet<Position>) -> i64 {
   let mut perimeter = 0;
   let _plant = grid[region.iter().next().unwrap().0][region.iter().next().unwrap().1];

   for &(row, col) in region {
       // Check each side of current position
       // Top
       if row == 0 || !region.contains(&(row-1, col)) {
           perimeter += 1;
       }
       // Bottom
       if row == grid.len()-1 || !region.contains(&(row+1, col)) {
           perimeter += 1;
       }
       // Left
       if col == 0 || !region.contains(&(row, col-1)) {
           perimeter += 1;
       }
       // Right
       if col == grid[0].len()-1 || !region.contains(&(row, col+1)) {
           perimeter += 1;
       }
   }

   perimeter
}