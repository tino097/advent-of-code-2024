use std::error::Error;
use std::collections::{HashSet, HashMap};

type Position = (i32, i32);
type Grid = Vec<Vec<char>>;

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
   let grid = parse_grid(input);
   let mut total = 0;
   let regions = get_regions(&grid);

   for (_, region) in regions {
       let mut convex_corners = 0;
       
       // Count convex corners
       for &(x, y) in &region {
           if !region.contains(&(x-1, y)) && !region.contains(&(x, y-1)) {
               convex_corners += 1;
           }
           if !region.contains(&(x+1, y)) && !region.contains(&(x, y-1)) {
               convex_corners += 1;
           }
           if !region.contains(&(x-1, y)) && !region.contains(&(x, y+1)) {
               convex_corners += 1;
           }
           if !region.contains(&(x+1, y)) && !region.contains(&(x, y+1)) {
               convex_corners += 1;
           }
       }

       // Count concave corners
       let mut concave_corners = HashSet::new();
       for &(x, y) in &region {
           if !region.contains(&(x-1, y)) && 
              region.contains(&(x-1, y-1)) && 
              region.contains(&(x, y-1)) {
               concave_corners.insert((x-1, y, "up-right"));
           }
           if !region.contains(&(x-1, y)) && 
              region.contains(&(x-1, y+1)) && 
              region.contains(&(x, y+1)) {
               concave_corners.insert((x-1, y, "down-right")); 
           }
           if !region.contains(&(x+1, y)) && 
              region.contains(&(x+1, y-1)) && 
              region.contains(&(x, y-1)) {
               concave_corners.insert((x+1, y, "up-left"));
           }
           if !region.contains(&(x+1, y)) && 
              region.contains(&(x+1, y+1)) && 
              region.contains(&(x, y+1)) {
               concave_corners.insert((x+1, y, "down-left"));
           }
       }

       total += (convex_corners + concave_corners.len() as i32) as i64 * region.len() as i64;
   }

   Ok(total)
}

fn get_regions(grid: &Grid) -> HashMap<i32, HashSet<Position>> {
   let mut regions = HashMap::new();
   let mut tiles_map = HashMap::new();
   let mut region_number = 0;

   for y in 0..grid.len() {
       for x in 0..grid[0].len() {
           if tiles_map.contains_key(&(x as i32, y as i32)) {
               continue;
           }

           region_number += 1;
           let mut queue = vec![(x as i32, y as i32)];
           let mut visited = HashSet::new();
           let c = grid[y][x];

           while !queue.is_empty() {
               let (cx, cy) = queue.remove(0);
               if visited.contains(&(cx, cy)) {
                   continue;
               }

               visited.insert((cx, cy));
               
               for (nx, ny) in [(cx, cy-1), (cx-1, cy), (cx, cy+1), (cx+1, cy)] {
                   if nx < 0 || nx >= grid[0].len() as i32 || 
                      ny < 0 || ny >= grid.len() as i32 {
                       continue;
                   }
                   if grid[ny as usize][nx as usize] != c {
                       continue;
                   }
                   queue.push((nx, ny));
               }
           }

           regions.insert(region_number, visited.clone());
           for pos in visited {
               tiles_map.insert(pos, region_number);
           }
       }
   }

   regions
}

fn parse_grid(input: &str) -> Grid {
   input.lines()
       .map(|line| line.chars().collect())
       .collect()
}